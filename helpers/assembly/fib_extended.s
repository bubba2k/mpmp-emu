# Compute the 15th fib number...
# Result is stored in %reg5
# Should be 233

main: # Setup
  ldc %reg0 1
  ldc %reg1 1
  ldc %reg2 11
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
