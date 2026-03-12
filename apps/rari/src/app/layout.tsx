import type { LayoutProps, Metadata } from 'rari'

export default function RootLayout({ children }: LayoutProps) {
  return <>{children}</>
}

export const metadata: Metadata = {
  title: 'rari Framework Benchmark',
  description: 'Server Component Performance Testing Suite',
}
