setup:
	ldc %reg2 0x8001		# Load TTY clear address
	st	%reg2 %reg0			# Write to 0x8001 to clear TTY

	ldc %reg0 65			# Load ascii code 'A'
	ldc %reg1 91			# Load ascii code one *after* 'Z'
	ldc %reg2 0x8000	# Load TTY write address
loop:
	st  %reg2 %reg0			# Write the character
	inc	%reg0				    # Increment character
	tst %reg0 %reg1			# Check if we are at Z already
	jnzr loop
  hlt
