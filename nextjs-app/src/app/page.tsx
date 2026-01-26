import Counter from '../components/Counter'
import EnvTestComponent from '../components/EnvTestComponent'
import FetchExample from '../components/FetchExample'
import Markdown from '../components/Markdown'
import ServerWithClient from '../components/ServerWithClient'
import ShoppingList from '../components/ShoppingList'
import TestComponent from '../components/TestComponent'
import WhatsHot from '../components/WhatsHot'

export const revalidate = false

export default function Home() {
  return (
    <div className="min-h-screen bg-linear-to-br from-blue-50 to-indigo-100 py-8 px-4">
      <div className="max-w-6xl mx-auto space-y-8">
        <div className="bg-white rounded-xl p-8 shadow-sm border border-gray-200 text-center">
          <h1 className="text-4xl font-bold text-gray-900 mb-4">
            🟢 Next.js Framework Benchmark
          </h1>
          <p className="text-xl text-gray-600 mb-6">
            Server Component Performance Testing Suite
          </p>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 bg-green-50 p-6 rounded-lg">
            <div className="text-center">
              <div className="text-2xl font-bold text-green-600">Node.js</div>
              <div className="text-sm text-green-700">Runtime Engine</div>
            </div>
            <div className="text-center">
              <div className="text-2xl font-bold text-purple-600">React</div>
              <div className="text-sm text-purple-700">Server Components</div>
            </div>
            <div className="text-center">
              <div className="text-2xl font-bold text-blue-600">App</div>
              <div className="text-sm text-blue-700">Router</div>
            </div>
          </div>
        </div>

        <div className="bg-white rounded-xl p-8 shadow-sm border border-gray-200">
          <h2 className="text-2xl font-bold text-gray-900 mb-6 text-center">🎯 Server Components Showcase</h2>
          <p className="text-gray-600 text-center mb-8">All components rendered server-side for optimal performance benchmarking</p>

          <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-2">
            <div className="bg-gray-50 p-6 rounded-lg border border-gray-200">
              <h3 className="text-lg font-semibold text-gray-900 mb-4 text-center">Counter</h3>
              <Counter />
            </div>

            <div className="bg-gray-50 p-6 rounded-lg border border-gray-200">
              <h3 className="text-lg font-semibold text-gray-900 mb-4 text-center">Test Component</h3>
              <TestComponent />
            </div>

            <div className="bg-gray-50 p-6 rounded-lg border border-gray-200">
              <h3 className="text-lg font-semibold text-gray-900 mb-4 text-center">Shopping List</h3>
              <ShoppingList />
            </div>

            <div className="bg-gray-50 p-6 rounded-lg border border-gray-200">
              <h3 className="text-lg font-semibold text-gray-900 mb-4 text-center">What's Hot</h3>
              <WhatsHot />
            </div>

            <div className="bg-gray-50 p-6 rounded-lg border border-gray-200">
              <h3 className="text-lg font-semibold text-gray-900 mb-4 text-center">Environment Test</h3>
              <EnvTestComponent />
            </div>

            <div className="bg-gray-50 p-6 rounded-lg border border-gray-200">
              <h3 className="text-lg font-semibold text-gray-900 mb-4 text-center">Fetch Example</h3>
              <FetchExample />
            </div>

            <div className="bg-gray-50 p-6 rounded-lg border border-gray-200">
              <h3 className="text-lg font-semibold text-gray-900 mb-4 text-center">Server with Client</h3>
              <ServerWithClient />
            </div>

            <div className="bg-gray-50 p-6 rounded-lg border border-gray-200">
              <h3 className="text-lg font-semibold text-gray-900 mb-4 text-center">Markdown Renderer</h3>
              <Markdown />
            </div>
          </div>
        </div>

        <div className="bg-linear-to-r from-green-50 to-blue-50 rounded-xl p-8 border border-green-200">
          <h2 className="text-2xl font-bold text-gray-900 mb-4">📈 Benchmark Summary</h2>
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
            <div className="bg-white p-4 rounded-lg text-center">
              <div className="text-2xl font-bold text-green-600">8</div>
              <div className="text-sm text-gray-700">Components</div>
            </div>
            <div className="bg-white p-4 rounded-lg text-center">
              <div className="text-2xl font-bold text-blue-600">Static</div>
              <div className="text-sm text-gray-700">Rendering</div>
            </div>
            <div className="bg-white p-4 rounded-lg text-center">
              <div className="text-2xl font-bold text-purple-600">Server</div>
              <div className="text-sm text-gray-700">Components</div>
            </div>
            <div className="bg-white p-4 rounded-lg text-center">
              <div className="text-2xl font-bold text-orange-600">Pure</div>
              <div className="text-sm text-gray-700">SSR</div>
            </div>
          </div>
          <div className="mt-6 text-center">
            <p className="text-gray-600">
              Static server component rendering for accurate performance benchmarking.
              <br />
              <span className="text-sm text-gray-500">Powered by Next.js App Router</span>
            </p>
          </div>
        </div>
      </div>
    </div>
  )
}
