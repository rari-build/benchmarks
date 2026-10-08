import type { ReactNode } from 'react'

interface RootLayoutProps {
  readonly children: ReactNode
}

export function RootLayout({ children }: RootLayoutProps) {
  return (
    <html lang="en">
      <head />
      <body>{children}</body>
    </html>
  )
}
