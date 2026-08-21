// ====================================================================
// SOUL IDENTIFICATION RUNE (SIR) - BASE SLAB GENERATOR
// Aurphyx Primordial Standards (APS) - Parametric Specification V3
// Document: APS-SIR-SLAB-01 | Companion to APS-SIR-BOM-2026-V3
// Author: Audry (Aurphyx) | 2026-08-06
// ====================================================================
//
// DESIGN INTENT
// -------------
// Single large-format "ancient tablet" prototype.
// Blank monolithic stone slab with two precision pockets:
//   Left  : Diamond (Birthstone Chamber)
//   Right : Rounded Rectangle (SKIM / SoulKey Identification Mandala Chamber)
// Raised outer border + central vertical divider.
// Material target: Imperial Black / Dark Forest Nephrite Jade
//                  (or Absolute Black Gabbro / Spectrolite Labradorite)
//
// SCALE: Macro tablet intended to later showcase nano-scale SKIM technology.
// UNITS: All dimensions in millimeters (professional standard).
// ====================================================================

/* [Rune Dimensions (mm)] */
rune_width      = 348;     // Target ~13.70 in
rune_height     = 162;     // Target ~6.38 in
rune_thickness  = 28;      // Target ~1.10 in

/* [Inlay Pocket Depths (mm)] */
birthstone_depth = 15.5;   // Deep enough for substantial birthstone + seating
skim_depth       = 14.5;   // Fluidic / quartz cell depth

/* [Pocket Footprints (mm) — proportionally scaled from Canva visual balance] */
diamond_w   = 112;
diamond_h   = 115;
skim_w      = 138;
skim_h      = 128;

/* [Styling & Border (mm)] */
corner_radius   = 12.0;    // Outer corner radius
border_width    = 4.5;     // Raised outer trim width
border_height   = 2.2;     // Raised outer trim height above Face A
central_bar_w   = 4.5;     // Central vertical divider width
central_bar_h   = 1.8;     // Slightly lower than outer border for hierarchy

/* [Rendering] */
$fn = 128;                 // High smoothness for large curved features

// --- MAIN ASSEMBLY ---
module SIR_Rune() {
    difference() {
        union() {
            // Base Stone Body
            rounded_slab(rune_width, rune_height, rune_thickness, corner_radius);
           
            // Raised Outer Border Trim
            translate([0, 0, rune_thickness])
                border_trim(rune_width, rune_height, corner_radius, border_width, border_height);
            
            // Central Vertical Divider
            translate([0, 0, rune_thickness])
                central_divider(rune_height, corner_radius, central_bar_w, central_bar_h, border_width);
        }
       
        // 1. Birthstone Diamond Pocket (Left)
        // Positioned to maintain healthy webs to outer border and central divider
        translate([-rune_width * 0.235, 0, rune_thickness - birthstone_depth + 0.05])
            diamond_pocket(diamond_w, diamond_h, birthstone_depth + border_height + 2);
           
        // 2. SKIM Chamber Pocket (Right)
        translate([rune_width * 0.215, 0, rune_thickness - skim_depth + 0.05])
            skim_chamber(skim_w, skim_h, skim_depth + border_height + 2, 13);
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

// Raised Outer Trim (hollow frame)
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

// Central Vertical Divider Bar
module central_divider(height, radius, bar_w, bar_h, outer_border_w) {
    linear_extrude(height = bar_h) {
        square([bar_w, height - 2*outer_border_w - 4], center = true);
    }
}

// Diamond-Shaped Cavity for Birthstone (rhombus)
module diamond_pocket(dia_w, dia_h, depth) {
    linear_extrude(height = depth) {
        polygon(points = [
            [0,        dia_h/2],
            [dia_w/2,  0],
            [0,       -dia_h/2],
            [-dia_w/2, 0]
        ]);
    }
}

// SKIM Fluid / Quartz Chamber (Rounded Rectangle)
module skim_chamber(ch_w, ch_h, depth, corner_r) {
    linear_extrude(height = depth) {
        offset(r = corner_r) {
            square([ch_w - 2*corner_r, ch_h - 2*corner_r], center = true);
        }
    }
}

// --- RENDER ---
SIR_Rune();

// ====================================================================
// USAGE NOTES
// ====================================================================
// 1. Open in OpenSCAD or compatible CAD environment.
// 2. All parameters are in millimeters.
// 3. Export as STL / 3MF for CNC toolpath generation.
// 4. For production stone work: use diamond tooling, water or ultrasonic
//    coolant, and conservative feeds on Nephrite / Absolute Black.
// 5. This is a single prototype blank. Visual fidelity and dimensional
//    accuracy take priority over speed.
// 6. Minimum wall / web integrity is the fabricator's responsibility
//    (see BOM structural notes).
// ====================================================================
