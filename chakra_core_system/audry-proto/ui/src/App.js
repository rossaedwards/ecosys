fetch(`${process.env.REACT_APP_API_URL}/shards`)
import React, { useEffect, useState } from "react";
import Graph from "./Graph";

function App() {
  const [shards, setShards] = useState([]);

  useEffect(() => {
    fetch("http://localhost:5000/shards")
      .then(res => res.json())
      .then(data => setShards(Object.entries(data)));
  }, []);

  return (
    <div>
      <h1>Shard Player</h1>
      <ul>
        {shards.map(([id, shard]) => (
          <li key={id}>
            {id}: {shard.description}
            <audio controls src={`http://localhost:5000/shards/${id}`} />
          </li>
        ))}
      </ul>
      <Graph />
    </div>
  );
}

export default App;
import React, { useEffect, useState } from "react";
import Graph from "./Graph";