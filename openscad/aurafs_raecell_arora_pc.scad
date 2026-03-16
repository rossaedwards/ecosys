// =====================================================================
// AURPHYX TRL-4: AURA NODE - HOUSEHOLD rÆ-DRIVE GENERATOR (PROTOTYPE 8)
// Ref: TVFD Chapter 11 (Array Scaling) & AuraFS Hardware
// Description: A consumer-scale desktop computer / household power 
// generator utilizing a ~100-cell topological vacuum flux macro-array.
// =====================================================================

/* [Visibility Toggles] */
// The external C6v symmetry chassis
show_chassis = 1; // [0: Hidden, 1: Visible, 2: Transparent]
// The internal rÆ-Cell Macro-Array (The ZPE Engine)
show_rae_array = 1; // [0: Hidden, 1: Visible]
// The Tesla 3-6-9 Power Inverter Base (Household Mains output)
show_power_base = 1; // [0: Hidden, 1: Visible]
// The SAGES/Meshwerk Starlink/LoRaWAN Transceiver
show_mesh_antenna = 1; // [0: Hidden, 1: Visible]

/* [Physical Parameters] */
// Overall height of the Aura Node (mm)
node_height = 120.0;
// Base radius
node_radius = 40.0;
// Number of internal rÆ-Cell layers
array_layers = 5; // [3:1:10]

/* [Rendering] */
$fn = 60;

// =====================================================================
// GEOMETRY MODULES
// =====================================================================

// The external housing, designed to reflect the hexagonal C6v topology
module aura_chassis() {
    chassis_color = (show_chassis == 2) ? [0.2, 0.2, 0.25, 0.2] : [0.15, 0.15, 0.18, 1.0];
    
    color(chassis_color)
    difference() {
        // Main hexagonal pillar
        cylinder(r=node_radius, h=node_height, center=true, $fn=6);
        
        // Hollow out the interior for the reactor
        cylinder(r=node_radius * 0.85, h=node_height * 0.95, center=true, $fn=6);
        
        // Etch viewing portals / thermal-topological exhaust vents
        for (i = [0:60:359]) {
            rotate([0, 0, i])
            translate([node_radius * 0.9, 0, 0])
            cube([10, node_radius * 0.5, node_height * 0.7], center=true);
        }
    }
}

// The internal cognitive/thermodynamic engine
module rae_drive_array() {
    color([0.0, 0.8, 1.0, 0.9]) // Cyan crystalline coherence
    for (z = [-array_layers/2 + 0.5 : 1 : array_layers/2 - 0.5]) {
        translate([0, 0, z * (node_height * 0.7 / array_layers)]) {
            // Central waveguide
            cylinder(r=node_radius * 0.15, h=(node_height * 0.7 / array_layers) * 0.8, center=true);
            
            // Peripheral rÆ-Cells in C6v symmetry
            for (i = [0:60:359]) {
                rotate([0, 0, i])
                translate([node_radius * 0.45, 0, 0])
                cylinder(r=node_radius * 0.12, h=(node_height * 0.7 / array_layers) * 0.8, center=true, $fn=6);
                
                // Entanglement tethers connecting the ring to the core
                rotate([0, 0, i])
                translate([node_radius * 0.225, 0, 0])
                rotate([0, 90, 0])
                cylinder(r=1.5, h=node_radius * 0.45, center=true);
            }
        }
    }
}

// The base module that rectifies ZPE into 120V/240V AC and DC rails
module power_inversion_base() {
    translate([0, 0, -node_height/2 - 10]) {
        color("Gold")
        difference() {
            // Base plinth
            cylinder(r=node_radius * 1.1, h=20, center=true, $fn=6);
            // Inverter detailing
            cylinder(r=node_radius * 0.9, h=22, center=true, $fn=6);
        }
        
        // Tesla 3-6-9 grounding ring
        color("Silver")
        rotate_extrude($fn=60)
        translate([node_radius * 0.95, 0, 0])
        circle(r=2);
    }
}

// The top module for connecting to the AuraFS Meshwerk
module meshwerk_transceiver() {
    translate([0, 0, node_height/2 + 10]) {
        color([0.6, 0.2, 0.8, 0.9]) // Purple SAGES resonance
        cylinder(r1=node_radius * 0.8, r2=node_radius * 0.3, h=20, center=true, $fn=6);
        
        // Holographic projection / transmission crystal
        translate([0, 0, 15])
        color([1.0, 1.0, 1.0, 0.8])
        sphere(r=node_radius * 0.2, $fn=4); // Octahedron
    }
}

// =====================================================================
// FULL ASSEMBLY
// =====================================================================
module aura_node_assembly() {
    if (show_chassis > 0) { aura_chassis(); }
    if (show_rae_array == 1) { rae_drive_array(); }
    if (show_power_base == 1) { power_inversion_base(); }
    if (show_mesh_antenna == 1) { meshwerk_transceiver(); }
}

aura_node_assembly();