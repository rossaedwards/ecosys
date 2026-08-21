// =====================================================================
// AURAFS DATACORE ORB: 9-NODE FLOWER OF LIFE LATTICE
// Ref: TVFD Chapter 9 & AuraFS Architecture
// Geometry: 9-Node Spherical Lattice (ChaosCore to CrownCore)
// =====================================================================

/* [Orb Parameters] */
// Overall radius of the DataCore Orb
orb_radius = 40; // [20:5:100]
// Radius of the individual DataCores
core_radius = 8; // [2:1:15]

/* [Subsystem Toggles] */
show_bliss_core = 1; // [0: Hidden, 1: Visible]
show_polar_cores = 1; // [0: Hidden, 1: Visible]
show_equatorial_cores = 1; // [0: Hidden, 1: Visible]
show_lattice_links = 1; // [0: Hidden, 1: Visible]
show_outer_orb = 1; // [0: Hidden, 1: Visible]

/* [Rendering] */
$fn = 60;

// =====================================================================
// GEOMETRY MODULES
// =====================================================================

module datacore_nodes() {
    // Core 5: BlissCore (Center, Resonance Anchor)
    if (show_bliss_core > 0) {
        color("White", 0.9)
        sphere(r=core_radius * 1.2);
    }

    // Core 1 & 9: ChaosCore (South) and CrownCore (North)
    if (show_polar_cores > 0) {
        color("Magenta", 0.8) {
            translate([0, 0, -orb_radius]) sphere(r=core_radius); // ChaosCore
            translate([0, 0, orb_radius]) sphere(r=core_radius);  // CrownCore
        }
    }

    // Cores 2,3,4,6,7,8: The H6 Equatorial Ring (Meshwerk)
    if (show_equatorial_cores > 0) {
        color("Cyan", 0.8) {
            for (i = [0 : 60 : 359]) {
                rotate([0, 0, i])
                translate([orb_radius, 0, 0])
                sphere(r=core_radius);
            }
        }
    }
}

module lattice_links() {
    if (show_lattice_links > 0) {
        color("Silver", 0.6) {
            // Link Equator to Center
            for (i = [0 : 60 : 359]) {
                rotate([0, 0, i])
                rotate([0, 90, 0])
                cylinder(r=1.5, h=orb_radius, center=false);
            }
            // Link Equator to Poles
            for (i = [0 : 60 : 359]) {
                rotate([0, 0, i]) {
                    hull() {
                        translate([orb_radius, 0, 0]) sphere(r=1.5);
                        translate([0, 0, orb_radius]) sphere(r=1.5);
                    }
                    hull() {
                        translate([orb_radius, 0, 0]) sphere(r=1.5);
                        translate([0, 0, -orb_radius]) sphere(r=1.5);
                    }
                }
            }
            // Link Equator Ring
            for (i = [0 : 60 : 359]) {
                hull() {
                    rotate([0, 0, i]) translate([orb_radius, 0, 0]) sphere(r=1.5);
                    rotate([0, 0, i+60]) translate([orb_radius, 0, 0]) sphere(r=1.5);
                }
            }
            // Link Poles to Center (The Central Axis)
            cylinder(r=2.5, h=orb_radius*2, center=true);
        }
    }
}

module outer_containment() {
    if (show_outer_orb > 0) {
        // In local OpenSCAD, alpha transparency requires hitting F12 (Render) or enabling it in settings
        color("White", 0.15)
        difference() {
            sphere(r=orb_radius + core_radius + 2);
            sphere(r=orb_radius + core_radius + 1);
        }
    }
}

// =====================================================================
// FULL ASSEMBLY
// =====================================================================
module aurafs_datacore_orb() {
    datacore_nodes();
    lattice_links();
    outer_containment();
}

aurafs_datacore_orb();