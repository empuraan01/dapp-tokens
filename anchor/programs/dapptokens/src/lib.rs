use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, Mint, Token, TokenAccount as SplTokenAccount, Transfer, TransferChecked,
};

declare_id!("JAVuBXeBZqXNtS73azhBDAoYaaAFfo4gWXoZe2e7Jf8H");

#[program]
pub mod dapptokens {
    use super::*;

    
    pub fn transfer_tokens(
        ctx: Context<TransferTokens>,
        amount: u64,
    ) -> Result<()> {

        require!(amount > 0, PaymentError::InvalidAmount);

        let from_account = &ctx.accounts.from_token_account;
        let to_account = &ctx.accounts.to_token_account;

        require!(
            from_account.mint == to_account.mint,
            PaymentError::MintMismatch
        );

        require!(
            from_account.amount >= amount,
            PaymentError::InsufficientFunds
        );

        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.from_token_account.to_account_info(),
                to: ctx.accounts.to_token_account.to_account_info(),
                authority: ctx.accounts.from_authority.to_account_info(),
            },
        );

        token::transfer(transfer_ctx, amount)?;

        emit!(TokenTransferEvent {
            from: ctx.accounts.from_authority.key(),
            to: ctx.accounts.to_authority.key(),
            mint: from_account.mint,
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn transfer_tokens_checked(
        ctx: Context<TransferTokensChecked>,
        amount: u64,
        decimals: u8,
    ) -> Result<()> {
        require!(amount > 0, PaymentError::InvalidAmount);

        let from_account = &ctx.accounts.from_token_account;
        let to_account = &ctx.accounts.to_token_account;
        let mint = &ctx.accounts.mint;

        require!(
            mint.decimals == decimals,
            PaymentError::InvalidDecimals
        );

        require!(
            from_account.amount >= amount,
            PaymentError::InsufficientFunds
        );

        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.from_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.to_token_account.to_account_info(),
                authority: ctx.accounts.from_authority.to_account_info(),
            },
        );

        token::transfer_checked(transfer_ctx, amount, decimals)?;

        emit!(TokenTransferEvent {
            from: ctx.accounts.from_authority.key(),
            to: ctx.accounts.to_authority.key(),
            mint: mint.key(),
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn get_payment_info(ctx: Context<GetPaymentInfo>) -> Result<PaymentInfo> {
        let from_account = &ctx.accounts.from_token_account;
        let to_account = &ctx.accounts.to_token_account;
        let mint = &ctx.accounts.mint;

        Ok(PaymentInfo {
            from_balance: from_account.amount,
            to_balance: to_account.amount,
            mint: mint.key(),
            decimals: mint.decimals,
            is_valid_transfer: from_account.mint == to_account.mint && from_account.mint == mint.key(),
        })
    }
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    pub from_authority: Signer<'info>,

    #[account(
        mut,
        constraint = from_token_account.owner == from_authority.key() @ PaymentError::InvalidOwner
    )]
    pub from_token_account: Account<'info, SplTokenAccount>,

    #[account(mut)]
    pub to_token_account: Account<'info, SplTokenAccount>,

    pub to_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferTokensChecked<'info> {
    pub from_authority: Signer<'info>,

    #[account(
        mut,
        constraint = from_token_account.owner == from_authority.key() @ PaymentError::InvalidOwner,
        constraint = from_token_account.mint == mint.key() @ PaymentError::MintMismatch
    )]
    pub from_token_account: Account<'info, SplTokenAccount>,

    #[account(
        mut,
        constraint = to_token_account.mint == mint.key() @ PaymentError::MintMismatch
    )]
    pub to_token_account: Account<'info, SplTokenAccount>,

    pub mint: Account<'info, Mint>,

    pub to_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct GetPaymentInfo<'info> {
    pub from_token_account: Account<'info, SplTokenAccount>,

    pub to_token_account: Account<'info, SplTokenAccount>,

    pub mint: Account<'info, Mint>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PaymentInfo {
    pub from_balance: u64,
    pub to_balance: u64,
    pub mint: Pubkey,
    pub decimals: u8,
    pub is_valid_transfer: bool,
}

#[event]
pub struct TokenTransferEvent {
    pub from: Pubkey,
    pub to: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}


#[error_code]
pub enum PaymentError {
    #[msg("Invalid amount: amount must be greater than 0")]
    InvalidAmount,
    #[msg("Insufficient funds: sender does not have enough tokens")]
    InsufficientFunds,
    #[msg("Mint mismatch: source and destination accounts must use the same token mint")]
    MintMismatch,
    #[msg("Invalid owner: signer is not the owner of the source token account")]
    InvalidOwner,
    #[msg("Invalid decimals: provided decimals do not match the mint")]
    InvalidDecimals,
}

