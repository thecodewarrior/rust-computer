#cpudef {
    #bits 8

    nop -> 0[7:0]
    pop -> 1[7:0]
    dup -> 2[7:0]
    push {value} -> 3[7:0] @ value[31:0]
    uadd -> 4[7:0] 
    usub -> 5[7:0] 
    umul -> 6[7:0] 
    udiv -> 7[7:0] 
    urem -> 8[7:0] 
    jmp {label} -> 9[7:0] @ label[31:0]
    jmp_ez {label} -> 10[7:0] @ label[31:0]
    jmp_nz {label} -> 11[7:0] @ label[31:0]
}