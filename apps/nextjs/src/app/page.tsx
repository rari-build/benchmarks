import HomePage from '@benchmark/shared/components/HomePage'

export const revalidate = false

export default function Home() {
  return (
    <HomePage
      framework={{
        name: 'Next.js',
        emoji: '🟢',
        runtime: 'Node.js',
        runtimeColor: 'bg-green-50',
        feature1: 'Router',
        feature1Color: 'text-blue-700',
        feature2: 'App',
        feature2Color: 'text-blue-600',
        poweredBy: 'Next.js App Router',
      }}
    />
  )
}
