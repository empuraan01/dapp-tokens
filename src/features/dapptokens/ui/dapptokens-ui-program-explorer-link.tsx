import { useDapptokensProgramId } from '@/features/dapptokens/data-access/use-dapptokens-program-id'
import { AppExplorerLink } from '@/components/app-explorer-link'
import { ellipsify } from '@wallet-ui/react'

export function DapptokensUiProgramExplorerLink() {
  const programId = useDapptokensProgramId()

  return <AppExplorerLink address={programId.toString()} label={ellipsify(programId.toString())} />
}
