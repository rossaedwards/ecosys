import axios, { AxiosInstance } from 'axios';

export class AurphyxCasinoSDK {
  private api: AxiosInstance;

  constructor(baseURL: string) {
    this.api = axios.create({
      baseURL,
      headers: {
        'Content-Type': 'application/json',
      },
    });
  }

  async playGame(gameId: string, bet: number): Promise<any> {
    const response = await this.api.post('/casino/play', {
      gameId,
      bet,
    });
    return response.data;
  }

  async placeBet(eventId: string, amount: number): Promise<any> {
    const response = await this.api.post('/sportsbook/bet', {
      eventId,
      amount,
    });
    return response.data;
  }

  async getBalance(): Promise<number> {
    const response = await this.api.get('/wallet/balance');
    return response.data.balance;
  }
}

export default AurphyxCasinoSDK;

