// =====================================================================
// AURPHYX TRL-4: GOLDEN SILK ADIABATIC SPIRALS (PROTOTYPE 9)
// Ref: FTQC Braiding & Datacore-Orb Interconnects
// Description: Parametric 3D spiral waveguides for phase-protected 
// Neglecton transport and Fuxyez semantic routing.
// =====================================================================

/* [Braiding Parameters] */
spiral_turns = 1.5; // [0.5:0.5:5.0]
start_radius = 5.0; // [2.0:1.0:20.0]
end_radius = 25.0; // [10.0:1.0:50.0]
z_height = 40.0; // [10.0:5.0:100.0]

/* [Material Toggles] */
show_dlc_sheath = 1; // [0: Hidden, 1: Visible]
show_gold_core = 1; // [0: Hidden, 1: Visible]

/* [Rendering] */
$fn = 30; // Keep at 30 for hull loop performance

// =====================================================================
// GEOMETRY MODULES
// =====================================================================

module golden_silk_path(r_start, r_end, h, turns, thickness, mat_color, alpha) {
    // 1. Calculate resolution variables FIRST
    steps = turns * 40; 
    
    // 2. Apply color block to the resulting geometry
    color(mat_color, alpha) {
        for(i = [0 : steps - 1]) {
            t1 = i / steps;
            t2 = (i + 1) / steps;
            
            r1 = r_start + (r_end - r_start) * t1;
            r2 = r_start + (r_end - r_start) * t2;
            
            a1 = turns * 360 * t1;
            a2 = turns * 360 * t2;
            
            z1 = h * t1;
            z2 = h * t2;
            
            hull() {
                translate([r1 * cos(a1), r1 * sin(a1), z1]) sphere(r = thickness);
                translate([r2 * cos(a2), r2 * sin(a2), z2]) sphere(r = thickness);
            }
        }
    }
}

// =====================================================================
// ASSEMBLY (T1-T3 Trunk Configuration)
// =====================================================================
module braided_interconnect_assembly() {
    for (phase = [0, 120, 240]) {
        rotate([0, 0, phase]) {
            // Inner percolating gold nanoparticle core
            if (show_gold_core == 1) {
                golden_silk_path(start_radius, end_radius, z_height, spiral_turns, 0.8, "Gold", 1.0);
            }
            // Outer Diamond-Like Carbon (DLC) vacuum sheath
            if (show_dlc_sheath == 1) {
                golden_silk_path(start_radius, end_radius, z_height, spiral_turns, 1.6, [0.8, 0.9, 1.0], 0.3);
            }
        }
    }
}

braided_interconnect_assembly();