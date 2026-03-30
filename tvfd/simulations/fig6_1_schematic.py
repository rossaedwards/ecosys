import subprocess, os

scad_code = """
// x_Cell C6v Prototype Schematic
$fn = 60;
module rf_coil_ring() {
    for (i = [0:5]) rotate([0,0,i*60])
        translate([12,0,0]) cylinder(h=2, r=1.2, center=true);
}
module fpga_board() {
    color("green") cube([20,15,2], center=true);
}
module diamond_substrate() {
    color("lightblue", 0.7) cylinder(h=1, r=10, center=true);
}
module photodetector_array() {
    for (i = [0:5]) rotate([0,0,i*60])
        translate([8,0,3]) color("yellow")
        cylinder(h=1.5, r=1, center=true);
}
union() {
    translate([0,0,-1]) diamond_substrate();
    translate([0,0,4]) fpga_board();
    translate([0,0,1]) rf_coil_ring();
    photodetector_array();
}
"""

with open("x_Cell_fig6_1.scad", "w") as f:
    f.write(scad_code)

print("OpenSCAD file written: x_Cell_fig6_1.scad")
print("Run: openscad --render -o fig6_1_schematic.png x_Cell_fig6_1.scad")
