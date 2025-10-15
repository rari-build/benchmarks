import type { LayoutProps } from 'rari/client'

// eslint-disable-next-line react-refresh/only-export-components
export const metadata = {
  title: 'Rari Framework Benchmark',
  description: 'Server Component Performance Testing Suite',
}

export default function RootLayout({ children }: LayoutProps) {
  return (
    <html lang="en" className="h-full">
      <head>
        <meta charSet="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>Rari Framework Benchmark</title>
      </head>
      <body className="min-h-full bg-gray-50">
        <div id="root">
          {children}
        </div>
      </body>
    </html>
  )
}
