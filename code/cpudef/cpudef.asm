#cpudef {
    #bits 8

    nop -> 0[7:0]
    pop -> 1[7:0]
    dup -> 2[7:0]
    push {value} -> 3[7:0] @ value[31:0]
    uadd -> 4[7:0] @ 0[7:0]
    usub -> 4[7:0] @ 1[7:0]
    umul -> 4[7:0] @ 2[7:0]
    udiv -> 4[7:0] @ 3[7:0]
    urem -> 4[7:0] @ 4[7:0]

    ushl -> 4[7:0] @ 4[7:0]
    ushr -> 4[7:0] @ 6[7:0]
    unot -> 4[7:0] @ 7[7:0]
    uand -> 4[7:0] @ 8[7:0]
    uor  -> 4[7:0] @ 9[7:0]
    uxor -> 4[7:0] @ 10[7:0]

    imath -> 5[7:0]
    fmath -> 6[7:0]

    jmp {label} -> 7[7:0] @ label[31:0]

    ujmp_ez {label} -> 8[7:0] @ 0[7:0] @ label[31:0]
    ujmp_nz {label} -> 8[7:0] @ 1[7:0] @ label[31:0]
    ujmp_eq {label} -> 8[7:0] @ 2[7:0] @ label[31:0]
    ujmp_ne {label} -> 8[7:0] @ 3[7:0] @ label[31:0]
    ujmp_lt {label} -> 8[7:0] @ 4[7:0] @ label[31:0]
    ujmp_gt {label} -> 8[7:0] @ 5[7:0] @ label[31:0]
    ujmp_le {label} -> 8[7:0] @ 6[7:0] @ label[31:0]
    ujmp_ge {label} -> 8[7:0] @ 7[7:0] @ label[31:0]

    ijmp {label} -> 9[7:0] @ label[31:0]
    fjmp {label} -> 10[7:0] @ label[31:0]
}