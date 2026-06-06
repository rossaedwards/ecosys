# Create a simplified and more readable system-level integration block diagram
diagram_code = """
flowchart TD
    SENS["Sensors"]
    CTRL["Controller"]
    DASH["Monitor Dashboard"]
    
    AI["Acoustic Isolation"]
    EMI["EMI Shielding"]
    UHV["UHV Enclosure"]
    CC["Cryogenic Cooling"]
    VD["Vibration Damping"]
    
    QSI["Quantum System Interface"]
    
    %% Control connections
    SENS --> CTRL
    CTRL --> DASH
    
    %% Controller manages all isolation systems
    CTRL --> AI
    CTRL --> EMI
    CTRL --> UHV
    CTRL --> CC
    CTRL --> VD
    
    %% All isolation systems protect quantum interface
    AI --> QSI
    EMI --> QSI
    UHV --> QSI
    CC --> QSI
    VD --> QSI
    
    %% Direct sensor monitoring
    SENS --> QSI
    
    %% Styling for better contrast and readability
    style SENS fill:#1FB8CD,stroke:#000,stroke-width:2px,color:#000
    style CTRL fill:#DB4545,stroke:#000,stroke-width:2px,color:#fff
    style DASH fill:#2E8B57,stroke:#000,stroke-width:2px,color:#fff
    style AI fill:#5D878F,stroke:#000,stroke-width:2px,color:#fff
    style EMI fill:#D2BA4C,stroke:#000,stroke-width:2px,color:#000
    style UHV fill:#B4413C,stroke:#000,stroke-width:2px,color:#fff
    style CC fill:#964325,stroke:#000,stroke-width:2px,color:#fff
    style VD fill:#944454,stroke:#000,stroke-width:2px,color:#fff
    style QSI fill:#13343B,stroke:#000,stroke-width:3px,color:#fff
"""

# Create the diagram using the helper function with larger dimensions for better readability
png_path, svg_path = create_mermaid_diagram(diagram_code, 'system_integration_diagram.png', 'system_integration_diagram.svg', width=1600, height=1000)

print(f"Final improved diagram saved as: {png_path} and {svg_path}")