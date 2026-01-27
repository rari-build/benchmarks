import HomePage from '@benchmark/shared/components/HomePage'

export default function Page() {
  return (
    <HomePage
      framework={{
        name: 'rari',
        emoji: '🚀',
        runtime: 'Rust',
        runtimeColor: 'bg-blue-50',
        feature1: 'Configuration',
        feature1Color: 'text-green-700',
        feature2: 'Zero',
        feature2Color: 'text-green-600',
        poweredBy: 'rari\'s Rust runtime',
      }}
    />
  )
}

export const metadata = {
  title: 'rari Framework Benchmark',
  description: 'Server Component Performance Testing Suite',
}
