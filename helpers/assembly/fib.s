# Compute the fibonacci numbers
# Result is stored in %reg5

main: # Setup
  ldc %reg0 1
  ldc %reg1 1
  ldc %reg2 3
loop:
  add %reg0 %reg0 %reg1
  mov %reg5 %reg0
  dec %reg2
  jzr end
  add %reg1 %reg0 %reg1
  mov %reg5 %reg1
  dec %reg2
  jnzr loop
end:
  hlt
