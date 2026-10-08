import type { Metadata } from 'next'
import { RootLayout } from '@benchmark/shared/layout'
import { createSiteMetadata } from '@benchmark/shared/site-metadata'
import '@benchmark/shared/globals.css'

export const metadata: Metadata = createSiteMetadata('Next.js')

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return <RootLayout>{children}</RootLayout>
}
