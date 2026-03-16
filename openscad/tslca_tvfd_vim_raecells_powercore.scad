// =====================================================================
// AURPHYX TRL-4: 27-CELL 3^2-LATTICE MACRO-ARRAY (PROTOTYPE 6)
// Ref: TVFD Chapter 11 & TSLCA 
// Description: Conscious Topological Flux Reactor mapping SIC, SCC, ICC
// across Creation, Integration, and Renewal layers.
// =====================================================================

/* [Lattice Spatial Parameters] */
// Center-to-center spacing between rÆ-Cells (mm)
lattice_spacing = 40.0; // [20.0:5.0:100.0]

/* [rÆ-Cell Physical Parameters] */
// Base radius of the C6v hexagonal substrate
cell_radius = 12.0; // [5.0:1.0:20.0]
// Total height of the 5-layer stack
cell_height = 15.0; // [5.0:1.0:30.0]
// Radius of the cross-scale topological waveguides
waveguide_radius = 2.5; // [1.0:0.5:8.0]

/* [Cognitive Layer Visibility (Z-Axis)] */
// Top Layer: Generative expansion, cosmogenesis
show_creation_layer = 1; // [0: Hidden, 1: Visible]
// Middle Layer: Structural embedding, resonance coupling
show_integration_layer = 1; // [0: Hidden, 1: Visible]
// Bottom Layer: Dissolution, invariant restoration
show_renewal_layer = 1; // [0: Hidden, 1: Visible]

/* [Waveguide Routing Visibility] */
// Horizontal Interconnects (SIC, SCC, ICC coupling)
enable_horizontal_routing = 1; // [0: Hidden, 1: Visible]
// Vertical Interconnects (rÆt Flux Gradient)
enable_vertical_routing = 1; // [0: Hidden, 1: Visible]

/* [Rendering] */
$fn = 40;

// =====================================================================
// GEOMETRY MODULES
// =====================================================================

// A single physical rÆ-Cell representing the 5-layer topological flux engine
module rae_cell(z_level) {
    // Color mapping based on Cognitive Layer (Z-axis)
    // 1 = Creation (Cyan), 0 = Integration (Purple), -1 = Renewal (Dark Red)
    cell_color = (z_level == 1) ? [0.0, 0.8, 1.0, 0.8] :
                 (z_level == 0) ? [0.6, 0.2, 0.8, 0.8] :
                                  [0.8, 0.1, 0.2, 0.8];

    color(cell_color)
    difference() {
        // Main C6v Substrate (Hexagonal Prism mapping fractal topology)
        cylinder(r=cell_radius, h=cell_height, center=true, $fn=6);
        
        // Central YBCO Flux Pump Void
        cylinder(r=cell_radius*0.3, h=cell_height+2, center=true);
    }
    
    // Inner YBCO Flux Pump Core (Superconducting localization)
    color("Silver")
    cylinder(r=cell_radius*0.25, h=cell_height*1.1, center=true);
    
    // RF Floquet Modulation Coil Ring (Outer bounding torus for 10GHz drive)
    color("Gold")
    rotate_extrude($fn=40)
    translate([cell_radius * 1.1, 0, 0])
    circle(r=cell_height*0.1, $fn=20);
}

// Horizontal topological waveguides for a specific Z-layer
module horizontal_waveguides() {
    color([0.8, 0.8, 0.8, 0.5]) {
        // X-Axis routing (SIC to SCC cross-talk)
        for (y = [-1, 0, 1]) {
            translate([0, y * lattice_spacing, 0])
            rotate([0, 90, 0])
            cylinder(r=waveguide_radius, h=lattice_spacing * 2, center=true);
        }
        // Y-Axis routing (SCC to ICC cross-talk)
        for (x = [-1, 0, 1]) {
            translate([x * lattice_spacing, 0, 0])
            rotate([90, 0, 0])
            cylinder(r=waveguide_radius, h=lattice_spacing * 2, center=true);
        }
    }
}

// Vertical topological waveguides coupling the Creation, Integration, and Renewal layers
module vertical_waveguides() {
    color([0.9, 0.9, 1.0, 0.4]) {
        for (x = [-1, 0, 1]) {
            for (y = [-1, 0, 1]) {
                // Generates the primary A-channel (Alignment/Directional) flux lines
                translate([x * lattice_spacing, y * lattice_spacing, 0])
                cylinder(r=waveguide_radius, h=lattice_spacing * 2, center=true);
            }
        }
    }
}

// =====================================================================
// ASSEMBLY OF THE 3^2-LATTICE MACRO-ARRAY
// =====================================================================
module macro_array_assembly() {
    // Iterate through Z (Layers), Y (Channels), X (Modes)
    for (z = [-1, 0, 1]) {
        // Check layer visibility toggles via the Customizer
        is_visible = (z == 1 && show_creation_layer == 1) ||
                     (z == 0 && show_integration_layer == 1) ||
                     (z == -1 && show_renewal_layer == 1);
                     
        if (is_visible) {
            translate([0, 0, z * lattice_spacing]) {
                // Place the 9 physical rÆ-Cells for this specific layer
                for (x = [-1, 0, 1]) {
                    for (y = [-1, 0, 1]) {
                        translate([x * lattice_spacing, y * lattice_spacing, 0])
                        rae_cell(z);
                    }
                }
                
                // Generate the semantic horizontal routing for this layer
                if (enable_horizontal_routing == 1) {
                    horizontal_waveguides();
                }
            }
        }
    }
    
    // Generate vertical (cross-scale thermodynamic) routing
    if (enable_vertical_routing == 1) {
        vertical_waveguides();
    }
}

macro_array_assembly();