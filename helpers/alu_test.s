  ldc %reg0 0x5
  ldc %reg1 0x1
  ldc %reg2 0x80
  add %reg3 %reg1 %reg1
  sub %reg3 %reg0 %reg1
  inc %reg3
  mov %reg3 %reg0
  tst %reg1 %reg2
  add3 %reg3 %reg0 %reg1 %reg2
  shl %reg3 %reg3 %reg1
  shr %reg3 %reg3 %reg1
  dec %reg3
  and %reg4 %reg3 %reg0
  or  %reg4 %reg3 %reg2
  not %reg1 %reg1
  ldc %reg5 0xffff
  inc %reg5
  dec %reg5
  add %reg5 %reg5 %reg1
  sub %reg5 %reg5 %reg1
