import { Suspense } from 'react'

const delays = {
  fast: [100, 100, 100, 100, 100],
  medium: [500, 500, 500],
  slow: [1000, 1000],
}

interface HeaderProps {
  readonly framework: string
}

async function Header({ framework }: HeaderProps) {
  await new Promise<void>(resolve => {
    setTimeout(resolve, 1)
  })
  return <h1>{framework} Streaming Benchmark</h1>
}

interface CardProps {
  readonly delay: number
  readonly title: string
}

async function Card({ delay, title }: CardProps) {
  await new Promise<void>(resolve => {
    setTimeout(resolve, delay)
  })
  return (
    <div
      data-bench-stream="resolved"
      style={{ padding: 16, border: '1px solid #ccc', borderRadius: 8 }}
    >
      <h3>{title}</h3>
      <p>
        Loaded after {delay}
        ms
      </p>
    </div>
  )
}

function Skeleton() {
  return (
    <div
      style={{
        padding: 16,
        border: '1px solid #eee',
        borderRadius: 8,
        background: '#f5f5f5',
      }}
    >
      <div style={{ height: 24, width: '60%', background: '#ddd', borderRadius: 4 }} />
      <div
        style={{
          height: 16,
          width: '40%',
          background: '#eee',
          borderRadius: 4,
          marginTop: 8,
        }}
      />
    </div>
  )
}

interface StreamItem {
  readonly delay: number
  readonly title: string
}

interface SkeletonCardsProps {
  readonly label: string
  readonly items: readonly StreamItem[]
}

function SkeletonCards({ label, items }: SkeletonCardsProps) {
  return (
    <>
      <h2>{label}</h2>
      {items.map(item => (
        <Suspense key={item.title} fallback={<Skeleton />}>
          <Card delay={item.delay} title={item.title} />
        </Suspense>
      ))}
    </>
  )
}

interface StreamPageProps {
  readonly framework: string
}

export default function StreamPage({ framework }: StreamPageProps) {
  return (
    <main>
      <Header framework={framework} />
      <SkeletonCards
        label="Fast Items"
        items={delays.fast.map((d, i) => ({
          delay: d,
          title: `Fast Item ${i + 1}`,
        }))}
      />
      <SkeletonCards
        label="Medium Items"
        items={delays.medium.map((d, i) => ({
          delay: d,
          title: `Medium Item ${i + 1}`,
        }))}
      />
      <SkeletonCards
        label="Slow Items"
        items={delays.slow.map((d, i) => ({
          delay: d,
          title: `Slow Item ${i + 1}`,
        }))}
      />
    </main>
  )
}
