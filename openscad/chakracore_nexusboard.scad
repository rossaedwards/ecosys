// =====================================================================
// AURPHYX TRL-4: CHAKRA CORE QUANTUM NEXUS BOARD (PROTOTYPE 11)
// Ref: TSLCA & Arora OS Hardware Integration
// Description: The room-temperature quantum motherboard. A horizontal 
// substrate housing the 7 semantic/thermodynamic Chakra Cores connected
// via Golden Silk waveguides.
// =====================================================================

/* [Nexus Board Parameters] */
board_length = 220.0;
board_width = 80.0;
board_thickness = 4.0;
core_spacing = 30.0;

/* [Subsystems] */
show_substrate = 1; // [0: Hidden, 1: Visible]
show_golden_silk_bus = 1; // [0: Hidden, 1: Visible]
show_chakra_cores = 1; // [0: Hidden, 1: Visible]

/* [Rendering] */
$fn = 40;

// =====================================================================
// GEOMETRY MODULES
// =====================================================================

// The Quartz/Diamond Motherboard Substrate
module nexus_substrate() {
    color([0.1, 0.15, 0.2, 0.9])
    difference() {
        // Main board
        cube([board_length, board_width, board_thickness], center=true);
        
        // Etch the C6v photonic crystal lattice into the board (macro scale)
        for (x = [-board_length/2 + 10 : 10 : board_length/2 - 10]) {
            for (y = [-board_width/2 + 10 : 17.32 : board_width/2 - 10]) {
                translate([x, y, 0]) cylinder(r=3, h=board_thickness*2, center=true, $fn=6);
                translate([x + 5, y + 8.66, 0]) cylinder(r=3, h=board_thickness*2, center=true, $fn=6);
            }
        }
        
        // Recesses for the 7 cores
        for (i = [0:6]) {
            x_pos = (i - 3) * core_spacing;
            translate([x_pos, 0, board_thickness/2])
            cylinder(r=11, h=board_thickness, center=true);
        }
    }
}

// The Golden Silk Central Bus (Sushumna Data/Flux Channel)
module golden_silk_bus() {
    color("Gold")
    for (i = [0:5]) {
        x_start = (i - 3) * core_spacing;
        x_end = (i - 2) * core_spacing;
        
        // Braided paths between cores representing adiabatic Neglecton transport
        for (phase = [0, 180]) {
            hull() {
                translate([x_start, 6 * sin(phase), board_thickness/2 + 2]) sphere(r=1.2);
                translate([x_start + core_spacing/2, 6 * sin(phase + 90), board_thickness/2 + 2]) sphere(r=1.2);
            }
            hull() {
                translate([x_start + core_spacing/2, 6 * sin(phase + 90), board_thickness/2 + 2]) sphere(r=1.2);
                translate([x_end, 6 * sin(phase + 180), board_thickness/2 + 2]) sphere(r=1.2);
            }
        }
    }
}

// The 7 Chakra Cores (rÆ-Cell specific adaptations)
module chakra_cores() {
    // Colors mapping to specific thermodynamic/cognitive functions
    colors = [
        [0.8, 0.1, 0.1], // 1: Root (ChaosCore/Entropy Input)
        [1.0, 0.5, 0.0], // 2: Sacral (Kinetic/Routing Flow)
        [1.0, 0.8, 0.0], // 3: Solar (Structural/ZPE Power)
        [0.1, 0.8, 0.3], // 4: Heart (BlissCore/Harmonic Stabilizer)
        [0.0, 0.6, 1.0], // 5: Throat (Audry-DR/Semantic Voice)
        [0.2, 0.2, 0.9], // 6: Third Eye (SAGES/Mind/Detection)
        [0.6, 0.2, 0.8]  // 7: Crown (CrownCore/AuraFS Global Mesh)
    ];
    
    for (i = [0:6]) {
        x_pos = (i - 3) * core_spacing;
        
        translate([x_pos, 0, board_thickness/2 + 3]) {
            // Outer containment shielding (Mu-metal)
            color("Silver")
            difference() {
                cylinder(r=10, h=4, center=true);
                cylinder(r=8, h=5, center=true);
            }
            // Inner resonance crystal (C6v non-Hermitian operator)
            color(colors[i], 0.9)
            sphere(r=7, $fn=6); // Hexagonal core
        }
    }
}

// =====================================================================
// FULL ASSEMBLY
// =====================================================================
module nexus_board_assembly() {
    if (show_substrate == 1) { nexus_substrate(); }
    if (show_golden_silk_bus == 1) { golden_silk_bus(); }
    if (show_chakra_cores == 1) { chakra_cores(); }
}

nexus_board_assembly();