#include "cpudef/cpudef.asm"

push 0x04
loop: 
    push 0x01
    usub
    dup
ujmp_nz loop