begin:
  jr main # Start in main procedure

puts:
  # Print a string to the terminal
  # args:   %reg0:  address of the first character of the string
  #         %reg1:  number of consecutive characters to print
  ldc %reg2 0x0
  tst %reg1 %reg2 # If number is 0, return right away
  jzr putsend
  ldc %reg3 0x8000  # Keep TTY address in reg3
putsloop:
  ld %reg2 %reg0  # Load char from string
  st %reg3 %reg2  # Put char to terminal
  inc %reg0       # Increment address
  dec %reg1       # Decrement iterator var
  jnzr putsloop
putsend:
  hlt

main:
  # Load the string into RAM
  ldc %reg1 0x0   # Load address

  ldc %reg0 72    # Load 'H'
  st %reg1 %reg0  # Store char
  inc %reg1       # Increment address
  ldc %reg0 101   # Load 'e' and so on...
  st %reg1 %reg0
  inc %reg1
  ldc %reg0 108
  st %reg1 %reg0
  inc %reg1
  ldc %reg0 108
  st %reg1 %reg0
  inc %reg1
  ldc %reg0 111
  st %reg1 %reg0
  inc %reg1
  ldc %reg0 32
  st %reg1 %reg0
  inc %reg1
  ldc %reg0 119  # 'w'
  st %reg1 %reg0
  inc %reg1
  ldc %reg0 111
  st %reg1 %reg0
  inc %reg1
  ldc %reg0 114
  st %reg1 %reg0
  inc %reg1
  ldc %reg0 108
  st %reg1 %reg0
  inc %reg1
  ldc %reg0 100
  st %reg1 %reg0
  inc %reg1
  ldc %reg0 33
  st %reg1 %reg0
  inc %reg1

  # Setup done. Call puts and go
  ldc %reg0 0x0
  ldc %reg1 12
  jr puts
  
