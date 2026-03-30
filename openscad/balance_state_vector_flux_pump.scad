// =====================================================================
// AURPHYX ZPE HARVESTER: CASIMIR FRACTAL PLATES & YBCO FLUX PUMP
// Ref: TVFD Chapter 10 (ZPE_Core)
// Target: 12% vacuum energy density suppression via ds=1.36 geometry
// =====================================================================

/* [Casimir Cavity Parameters] */
// Gap distance between parallel plates
plate_gap = 4.0; // [1.0:0.5:10.0]
// Fractal recursion depth for the plates
fractal_depth = 3; // [1:1:4]
// Base radius of the plates
plate_radius = 25;

/* [ZPE Extraction Subsystems] */
// Render the upper Casimir plate
show_top_plate = 1; // [0: Hidden, 1: Visible]
// YBCO Superconducting Flux Pump Core
enable_ybco_core = 1; // [0: Disabled, 1: Enabled]
// Tesla 3-6-9 Resonant Coil Array
enable_tesla_coils = 1; // [0: Disabled, 1: Enabled]

/* [Rendering] */
$fn = 60;

// =====================================================================
// GEOMETRY MODULES
// =====================================================================

// Recursive function using 3D shapes to prevent web-browser crashes
module sierpinski_hex_3d(radius, depth, h) {
    if (depth == 0) {
        cylinder(r=radius, h=h, center=true, $fn=6);
    } else {
        for (i = [0 : 60 : 359]) {
            rotate([0, 0, i])
            translate([radius/2, 0, 0])
            sierpinski_hex_3d(radius/3, depth-1, h);
        }
        sierpinski_hex_3d(radius/3, depth-1, h);
    }
}

// The Casimir Plate (acts as the boundary condition for the vacuum)
module casimir_plate(is_top) {
    z_offset = is_top ? plate_gap/2 + 0.5 : -plate_gap/2 - 0.5;
    // Make the top plate slightly transparent so we can see the trapped flux
    plate_color = is_top ? [0.7, 0.9, 1.0, 0.4] : [0.2, 0.2, 0.25, 0.9];
    
    color(plate_color)
    translate([0, 0, z_offset])
    difference() {
        cylinder(r=plate_radius, h=1, center=true, $fn=60);
        // Etch the fractal suppression zone (h=1.2 to ensure clean cuts)
        sierpinski_hex_3d(plate_radius * 0.85, fractal_depth, 1.2);
    }
}

// YBCO Superconducting Ring (Operates at cryogenic temperatures)
module ybco_flux_pump() {
    if (enable_ybco_core > 0) {
        // Main YBCO Bulk
        color("DarkSlateBlue")
        difference() {
            cylinder(r=plate_radius * 0.35, h=plate_gap * 0.8, center=true);
            cylinder(r=plate_radius * 0.25, h=plate_gap * 1.0, center=true);
        }
        // Magnetic pinning sites (Flux quantization zones)
        color("Cyan")
        for(i=[0:60:359]) {
            rotate([0, 0, i])
            translate([plate_radius * 0.3, 0, 0])
            cylinder(r=1.2, h=plate_gap * 0.85, center=true);
        }
    }
}

// Tesla 3-6-9 Resonant Coil Array (Couples ZPE to the FPGA rail)
module tesla_coils() {
    if (enable_tesla_coils > 0) {
        // 9 distinct coil assemblies arranged in C9v symmetry
        color("Gold")
        for (i = [0 : 40 : 359]) { 
            rotate([0, 0, i])
            translate([plate_radius * 0.65, 0, 0])
            difference() {
                cylinder(r=2.8, h=plate_gap * 0.9, center=true);
                cylinder(r=1.8, h=plate_gap * 1.1, center=true);
            }
        }
    }
}

// =====================================================================
// ASSEMBLY
// =====================================================================
module zpe_harvester_assembly() {
    casimir_plate(false); // Bottom boundary
    
    if (show_top_plate > 0) {
        casimir_plate(true);  // Top boundary
    }
    
    ybco_flux_pump();
    tesla_coils();
}

zpe_harvester_assembly();