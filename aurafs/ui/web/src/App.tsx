// afs/ui/web/src/App.tsx
import { Chat } from './components/chat/Chat'
import { Dashboard } from './components/dashboard/Dashboard'

export default function App() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900">
      <Dashboard />
      <Chat />
    </div>
  )
}
