main:
  ldc %reg2 0x8001
  inc %reg0
  add %reg0 %reg1 %reg2
  add3 %reg3 %reg4 %reg5 %reg2
  sub %reg0 %reg4 %reg5
  subc %reg2 %reg1 %reg4
  ldc %reg5 0x7832
  or %reg0 %reg1 %reg4
  nop
  jz %reg2
  jmp %reg3
  jcr 0
  st %reg3 %reg1
  ld %reg2 %reg5
