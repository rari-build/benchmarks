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

export default function StreamLoading() {
  return (
    <main>
      <h1>Loading...</h1>
      {Array.from({ length: 10 }, (_, i) => (
        <Skeleton key={i} />
      ))}
    </main>
  )
}
