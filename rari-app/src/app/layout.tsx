import type { LayoutProps } from 'rari'

// eslint-disable-next-line react-refresh/only-export-components
export const metadata = {
  title: 'rari Framework Benchmark',
  description: 'Server Component Performance Testing Suite',
}

export default function RootLayout({ children }: LayoutProps) {
  return <div className="min-h-full">{children}</div>
}
