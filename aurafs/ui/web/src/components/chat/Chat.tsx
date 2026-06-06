// afs/ui/web/src/components/chat/Chat.tsx
import { Button } from '../ui/button'
import { Input } from '../ui/input'

export function Chat() {
  return (
    <div className="max-w-2xl mx-auto p-8">
      <h1 className="text-4xl font-bold bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent mb-8">
        AuraFS AI Chat
      </h1>
      <div className="space-y-4">
        <Input placeholder="Ask AuraFS anything..." />
        <Button className="w-full bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700">
          Send → API
        </Button>
      </div>
    </div>
  )
}
