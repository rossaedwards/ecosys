// ====================================================================
// SOUL IDENTIFICATION RUNE (SIR) - BASE SLAB GENERATOR
// Aurphyx Primordial Standards (APS) - Parametric Specification
// ====================================================================

/* [Rune Dimensions (Inches)] */
rune_width_in      = 5.55;  // Default landscape width
rune_height_in     = 3.69;  // Default landscape height
rune_thickness_in  = 0.55;  // 0.55" depth for large SKIM chamber

/* [Inlay Pocket Depths (Inches)] */
birthstone_depth_in = 0.35; // Recess for diamond birthstone
skim_depth_in       = 0.40; // Deep chamber for SKIM fluid & quartz

/* [Styling & Resolution] */
corner_radius_in   = 0.30;  // Corner roundness of the main rune
border_width_in    = 0.15;  // Width of outer ornate border trim
border_height_in   = 0.05;  // Height of raised metallic trim above stone
$fn = 64;                   // Rendering smoothness

// --- CONVERSIONS (Inches to Millimeters for OpenSCAD rendering) ---
mm = 25.4;
w  = rune_width_in * mm;
h  = rune_height_in * mm;
t  = rune_thickness_in * mm;
r  = corner_radius_in * mm;
bs_d = birthstone_depth_in * mm;
skim_d = skim_depth_in * mm;
b_w = border_width_in * mm;
b_h = border_height_in * mm;

// --- MAIN ASSEMBLY ---
module SIR_Rune() {
    difference() {
        union() {
            // Base Stone Body
            rounded_slab(w, h, t, r);
            
            // Raised Outer Filigree/Border Trim
            translate([0, 0, t])
                border_trim(w, h, r, b_w, b_h);
        }
        
        // 1. Birthstone Diamond Pocket (Left Side)
        translate([-w*0.22, 0, t - bs_d + 0.01])
            diamond_pocket(w*0.32, h*0.60, bs_d + b_h + 1);
            
        // 2. SKIM Chamber Pocket (Right Side - Large for Bio-reactive Fluid)
        translate([w*0.20, 0, t - skim_d + 0.01])
            skim_chamber(w*0.42, h*0.72, skim_d + b_h + 1, 6);
    }
}

// --- HELPER MODULES ---

// Rounded Rectangular Slab
module rounded_slab(width, height, thickness, radius) {
    linear_extrude(height = thickness) {
        offset(r = radius) {
            square([width - 2*radius, height - 2*radius], center = true);
        }
    }
}

// Raised Outer Trim
module border_trim(width, height, radius, width_trim, height_trim) {
    linear_extrude(height = height_trim) {
        difference() {
            offset(r = radius)
                square([width - 2*radius, height - 2*radius], center = true);
            offset(r = radius - width_trim)
                square([width - 2*radius, height - 2*radius], center = true);
        }
    }
}

// Diamond-Shaped Cavity for Birthstone
module diamond_pocket(dia_w, dia_h, depth) {
    linear_extrude(height = depth) {
        polygon(points = [
            [0, dia_h/2],
            [dia_w/2, 0],
            [0, -dia_h/2],
            [-dia_w/2, 0]
        ]);
    }
}

// SKIM Fluid/Quartz Chamber (Rounded Pocket)
module skim_chamber(ch_w, ch_h, depth, corner_r) {
    linear_extrude(height = depth) {
        offset(r = corner_r) {
            square([ch_w - 2*corner_r, ch_h - 2*corner_r], center = true);
        }
    }
}

// Render the Rune
SIR_Rune();