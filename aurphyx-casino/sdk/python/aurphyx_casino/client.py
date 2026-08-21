"""Aurphyx Casino Python SDK Client"""

import requests
from typing import Dict, Any


class AurphyxCasinoSDK:
    """Client for Aurphyx Casino API"""

    def __init__(self, base_url: str):
        self.base_url = base_url
        self.session = requests.Session()

    def play_game(self, game_id: str, bet: int) -> Dict[str, Any]:
        """Play a casino game"""
        response = self.session.post(
            f"{self.base_url}/casino/play",
            json={"gameId": game_id, "bet": bet},
        )
        response.raise_for_status()
        return response.json()

    def get_balance(self) -> int:
        """Get wallet balance"""
        response = self.session.get(f"{self.base_url}/wallet/balance")
        response.raise_for_status()
        return response.json()["balance"]

