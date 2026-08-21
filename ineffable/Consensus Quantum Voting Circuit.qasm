OPENQASM 2.0;
include "qelib1.inc";
qreg q;
cx q, q;
cx q, q;
measure q -> c;