export default function Home() {
  return (
    <main className="min-h-screen bg-gradient-to-br from-purple-900 to-blue-900">
      <div className="container mx-auto px-4 py-16">
        <h1 className="text-6xl font-bold text-white text-center mb-8">
          🎰 Aurphyx Casino
        </h1>
        <p className="text-2xl text-white text-center mb-12">
          Provably Fair Multi-Chain Casino & Sportsbook
        </p>
        <div className="flex justify-center gap-4">
          <a
            href="/casino"
            className="px-8 py-4 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition"
          >
            Casino Games
          </a>
          <a
            href="/sportsbook"
            className="px-8 py-4 bg-green-600 text-white rounded-lg hover:bg-green-700 transition"
          >
            Sportsbook
          </a>
        </div>
      </div>
    </main>
  )
}

