// afs/ui/api-clients/api.ts
// Typed API client → afs/api endpoints

export interface ChatRequest {
  prompt: string
  model?: string
}

export interface ChatResponse {
  result: string
  tokens: number
}

export class AfsApi {
  private baseUrl = 'http://localhost:8080/api'

  async chat(request: ChatRequest): Promise<ChatResponse> {
    const res = await fetch(${this.baseUrl}/ai/chat, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(request)
    })
    return res.json()
  }
}

export const afsApi = new AfsApi()
