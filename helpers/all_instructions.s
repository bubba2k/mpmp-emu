 ldc %reg0 0x00
  ldc %reg2 -0x01
   ldc %reg1 42
    ldc %reg3 -1337
    add %reg0 %reg1 %reg2
    add3 %reg3 %reg4 %reg5 %reg2
    addc %reg1 %reg2 %reg3
    sub %reg0 %reg4 %reg5
    subc %reg0 %reg0 %reg0
    inc %reg0
    dec %reg0
    mul %reg0 %reg0 %reg0
    and %reg0 %reg0 %reg0
    or %reg0 %reg0 %reg0
    not %reg0 %reg0
    neg %reg0 %reg0
    xor %reg0 %reg0 %reg0
    xnor %reg0 %reg0 %reg0
    shl %reg0 %reg0 %reg0
    shr %reg0 %reg0 %reg0
    tst %reg0 %reg0
    mov %reg0 %reg0
    jmp %reg0
    jz %reg0
    jnz %reg0
    jc %reg0
    jrcon 2047
    jr -2047
jump:
    jzr jump
    jnzr 5
    jcr 5
    st %reg0 %reg1
    ld %reg5 %reg4
    nop
    hlt
