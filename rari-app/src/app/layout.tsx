import type { LayoutProps } from 'rari'

export default function RootLayout({ children }: LayoutProps) {
  return <div>{children}</div>
}

export const metadata = {
  title: 'rari Framework Benchmark',
  description: 'Server Component Performance Testing Suite',
}
