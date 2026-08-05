import { Suspense } from 'react'

const delays = {
  fast: [100, 100, 100, 100, 100],
  medium: [500, 500, 500],
  slow: [1000, 1000],
}

async function Header({ framework }: { framework: string }) {
  await new Promise(r => setTimeout(r, 1))
  return (
    <h1>
      {framework}
      {' '}
      Streaming Benchmark
    </h1>
  )
}

async function Card({ delay, title }: { delay: number, title: string }) {
  await new Promise(r => setTimeout(r, delay))
  return (
    <div
      data-bench-stream="resolved"
      style={{ padding: 16, border: '1px solid #ccc', borderRadius: 8 }}
    >
      <h3>{title}</h3>
      <p>
        Loaded after
        {' '}
        {delay}
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
      <div
        style={{ height: 24, width: '60%', background: '#ddd', borderRadius: 4 }}
      />
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

function SkeletonCards({
  label,
  items,
}: {
  label: string
  items: { delay: number, title: string }[]
}) {
  return (
    <>
      <h2>{label}</h2>
      {items.map((item, i) => (
        <Suspense key={i} fallback={<Skeleton />}>
          <Card delay={item.delay} title={item.title} />
        </Suspense>
      ))}
    </>
  )
}

interface StreamPageProps {
  framework: string
}

export default async function StreamPage({ framework }: StreamPageProps) {
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
