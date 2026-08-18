# Create a layered schematic for EMI Shielding Prototype module
diagram_code = """
flowchart TD
    A[Ambient EM Noise] --> B[Metallic Film<br/>Cu/Al Layer]
    B --> C[Mu-metal<br/>High-μ Layer]
    C --> D[Conductive<br/>Polymer/Meta]
    D --> E[EM Sensors &<br/>Active Cancel]
    E --> F[Quantum System<br/>Interface]
    
    %% Signal flow and feedback
    E --> G[Active EMI<br/>Cancellation]
    G --> H[Feedback Loop]
    H --> E
    
    %% Shield effectiveness arrows
    A -.->|Attenuated| B
    B -.->|Further<br/>Reduced| C
    C -.->|Magnetically<br/>Shielded| D
    D -.->|Composite<br/>Filtering| E
    E -.->|Protected| F
    
    %% Styling for layers
    classDef outerLayer fill:#FFCDD2
    classDef metalLayer fill:#B3E5EC
    classDef muLayer fill:#A5D6A7
    classDef polymerLayer fill:#FFEB8A
    classDef sensorLayer fill:#9FA8B0
    classDef quantumLayer fill:#E1BEE7
    classDef feedback fill:#FFE0B2
    
    class A outerLayer
    class B metalLayer
    class C muLayer
    class D polymerLayer
    class E,G sensorLayer
    class F quantumLayer
    class H feedback
"""

# Create the diagram and save as both PNG and SVG
png_path, svg_path = create_mermaid_diagram(
    diagram_code, 
    'emi_shielding_schematic.png',
    'emi_shielding_schematic.svg',
    width=1400,
    height=900
)

print(f"Diagram saved as PNG: {png_path}")
print(f"Diagram saved as SVG: {svg_path}")