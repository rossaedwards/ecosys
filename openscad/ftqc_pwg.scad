// =====================================================================
// FTQC PHOTONIC HEXAGONAL LATTICE (TRL-4 ADVANCED PROTOTYPE)
// Upgrades: Anderson Disorder, Waveguide Channels, Optical Mount
// =====================================================================

/* [Photonic Lattice Parameters] */
lattice_a = 15; 
r_over_a = 0.35; 
// Increased to 4 rings to clearly see the waveguides
num_rings = 4; 
substrate_z = 5; 
central_defect_multiplier = 0.0; 

/* [Anderson Localization (Disorder)] */
// Variance of random displacement. 0 = Perfect Crystal. >0 = Disordered.
disorder_variance = 0.0; // [0:0.1:5.0]
// Seed to lock the random generation
random_seed = 42; 

/* [Topological Waveguides] */
// Removes voids along the 6 axes to create solid physical "wires"
enable_waveguides = 0; // [0: Disabled, 1: Enabled]

/* [Physical Mounting Substrate] */
// Generates a square optical table mounting bracket with M4 screw holes
enable_mounting_bracket = 0; // [0: Disabled, 1: Enabled]

/* [Rendering] */
$fn = 40;

// =====================================================================
// COMPUTED VARIABLES & HASH FUNCTION
// =====================================================================
void_r = lattice_a * r_over_a;

// Unique seed generator for each lattice node
function hash(q, r, seed) = (q * 73856093 ^ r * 19349663 ^ seed);

// =====================================================================
// GEOMETRY GENERATION
// =====================================================================
module c6v_photonic_crystal() {
    difference() {
        
        // 1. The Substrate Body
        if (enable_mounting_bracket == 1) {
            color("GhostWhite", 0.8)
            cube([lattice_a*(num_rings*2.5), lattice_a*(num_rings*2.5), substrate_z], center=true);
        } else {
            color("GhostWhite", 0.8)
            cylinder(r = lattice_a * (num_rings + 0.8), h = substrate_z, center = true, $fn = 6);
        }

        // 2. The Void Array with Conditional Upgrades
        for (q = [-num_rings : num_rings]) {
            for (r = [max(-num_rings, -q - num_rings) : min(num_rings, -q + num_rings)]) {
                
                // Determine if this specific node lies on the 6 high-symmetry axes
                is_waveguide = (enable_waveguides == 1) && (q == 0 || r == 0 || q == -r);
                is_center = (q == 0 && r == 0);
                
                // If it is a waveguide, radius becomes 0 (solid material)
                base_r = is_center ? (void_r * central_defect_multiplier) : 
                         (is_waveguide ? 0 : void_r); 
                
                if (base_r > 0) {
                    // Apply Anderson Disorder to X and Y coordinates
                    dx = disorder_variance > 0 ? rands(-disorder_variance, disorder_variance, 1, hash(q,r,random_seed))[0] : 0;
                    dy = disorder_variance > 0 ? rands(-disorder_variance, disorder_variance, 1, hash(q,r,random_seed+1))[0] : 0;
                    
                    x_pos = lattice_a * (q + r / 2) + dx;
                    y_pos = lattice_a * (sqrt(3) / 2) * r + dy;
                    
                    translate([x_pos, y_pos, 0])
                    color("Black")
                    cylinder(r = base_r, h = substrate_z + 2, center = true);
                }
            }
        }
        
        // 3. Drill Optical Mounting Holes if Bracket is Enabled
        if (enable_mounting_bracket == 1) {
            mount_offset = lattice_a * num_rings;
            for (mx = [-1, 1]) {
                for (my = [-1, 1]) {
                    translate([mx * mount_offset, my * mount_offset, 0])
                    cylinder(r=2.2, h=substrate_z+2, center=true); // 4.4mm diameter for M4 screws
                }
            }
        }
    }
}

c6v_photonic_crystal();