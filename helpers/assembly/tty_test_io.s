main:

  ldc %reg1 0x8002
  ld %reg0 %reg1   # Get character from istream

  ldc %reg1 0x0     # If character is null, stream is
  tst %reg0 %reg1  # empty and we abort
  jzr end

  ldc %reg1 0x8000  # Else, print the character to
  st %reg1 %reg0   # ostream and jump to beginning
  jr main

end:
  hlt
