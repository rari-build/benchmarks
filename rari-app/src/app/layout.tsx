import type { LayoutProps } from 'rari'

export default function RootLayout({ children }: LayoutProps) {
  return <>{children}</>
}

export const metadata = {
  title: 'rari Framework Benchmark',
  description: 'Server Component Performance Testing Suite',
}
