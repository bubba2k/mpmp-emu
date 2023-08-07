# Compute the fifth 5 fibonacci number

main:
  # Setup
  ldc %reg0 1
  ldc %reg1 1
  ldc %reg2 5
loop:
  add %reg0 %reg0 %reg1
  dec %reg2 %reg2
  test %reg2 0
  jz end
  add %reg1 %reg0 %reg1
  dec %reg2 %reg2
  test %reg2
  jnz loop
end:
  halt
