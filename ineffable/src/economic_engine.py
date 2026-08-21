import random, asyncio, statistics

class OpulenceSimulation:
    def __init__(self, redistribution_rate=0.07):
        self.entities = {}
        self.redistribution_rate = redistribution_rate
        self.history = []

    def add_entity(self, name:str, liquidity:float, score:float):
        self.entities[name] = {"liq": liquidity, "score": score, "reward": 0}

    async def simulate_epoch(self, epoch:int):
        total_valuation = sum([ent["liq"] for ent in self.entities.values()])
        scores = sum([ent["score"] for ent in self.entities.values()])
        results = {}
        for name, ent in self.entities.items():
            proportion = (ent["score"] / scores) if scores else 0
            gain = total_valuation * self.redistribution_rate * proportion
            ent["liq"] += gain
            ent["reward"] = gain
            results[name] = gain
        self.history.append(results)

    async def run_simulation(self, epochs=10, delay=0.25):
        for epoch in range(epochs):
            await self.simulate_epoch(epoch)
            await asyncio.sleep(delay)
        print("Simulation completed successfully.")

    def audit_summary(self):
        for entity, data in self.entities.items():
            print(f"{entity}: Liquidity={data['liq']:.2f}, Reward={data['reward']:.2f}, Ethics={data['score']:.2f}")