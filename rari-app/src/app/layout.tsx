import type { LayoutProps } from 'rari/client'

export default function RootLayout({ children }: LayoutProps) {
  return (
    <html lang="en" className="h-full">
      <head>
        <meta charSet="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>Rari Framework Benchmark</title>
      </head>
      <body className="min-h-full bg-gray-50">
        {children}
      </body>
    </html>
  )
}

export const metadata = {
  title: 'Rari Framework Benchmark',
  description: 'Server Component Performance Testing Suite',
}
