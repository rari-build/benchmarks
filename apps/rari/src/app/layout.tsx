import type { LayoutProps, Metadata } from 'rari'
import { RootLayout } from '@benchmark/shared/layout'
import { createSiteMetadata } from '@benchmark/shared/site-metadata'
import '@benchmark/shared/globals.css'

export default function Layout({ children }: LayoutProps) {
  return <RootLayout>{children}</RootLayout>
}

export const metadata: Metadata = createSiteMetadata('rari')
