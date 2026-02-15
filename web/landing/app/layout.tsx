import type { Metadata } from 'next'
import type { ReactNode } from 'react'

export const metadata: Metadata = {
  title: 'Torvi - Idea Tournaments',
  description:
    'Create visual tournaments where the best ideas win through community voting.',
}

type Props = {
  children: ReactNode
}

const RootLayout = ({ children }: Props): ReactNode => {
  return children
}

export default RootLayout
