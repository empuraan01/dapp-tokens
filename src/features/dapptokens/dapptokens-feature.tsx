import { useSolana } from '@/components/solana/use-solana'
import { WalletButton } from '@/components/solana/solana-provider'
import { AppHero } from '@/components/app-hero'
import { DapptokensUiProgramExplorerLink } from './ui/dapptokens-ui-program-explorer-link'
import { DapptokensUiCreate } from './ui/dapptokens-ui-create'
import { DapptokensUiProgram } from '@/features/dapptokens/ui/dapptokens-ui-program'

export default function DapptokensFeature() {
  const { account } = useSolana()

  if (!account) {
    return (
      <div className="max-w-4xl mx-auto">
        <div className="hero py-[64px]">
          <div className="hero-content text-center">
            <WalletButton />
          </div>
        </div>
      </div>
    )
  }

  return (
    <div>
      <AppHero title="Dapptokens" subtitle={'Run the program by clicking the "Run program" button.'}>
        <p className="mb-6">
          <DapptokensUiProgramExplorerLink />
        </p>
        <DapptokensUiCreate />
      </AppHero>
      <DapptokensUiProgram />
    </div>
  )
}
