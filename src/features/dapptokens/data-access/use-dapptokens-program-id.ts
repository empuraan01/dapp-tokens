import { useSolana } from '@/components/solana/use-solana'
import { useMemo } from 'react'
import { getDapptokensProgramId } from '@project/anchor'

export function useDapptokensProgramId() {
  const { cluster } = useSolana()

  return useMemo(() => getDapptokensProgramId(cluster.id), [cluster])
}
