if __name__ == "__main__":
    sim = OpulenceSimulation(0.05)
    sim.add_entity("Tesla", 10000, 0.92)
    sim.add_entity("SpaceX", 15000, 0.88)
    sim.add_entity("Neuralink", 7000, 0.83)
    sim.add_entity("The Boring Company", 4500, 0.70)
    asyncio.run(sim.run_simulation(epochs=12))
    sim.audit_summary()