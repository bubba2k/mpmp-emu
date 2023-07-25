setup:
	ldc %reg2 0x8001		# Load TTY clear address
	ldc %reg0 0
	st	%reg2 %reg0			# Write to 0x8001 to clear TTY

	ldc %reg0 65			# Load ascii code 'A'
	ldc %reg1 90			# Load ascii code 'Z'
	ldc %reg2 0x8000		# Load TTY write address
loop:
	st  %reg2 %reg0			# Write the character
	inc	%reg0				# Increment character
	tst %reg0 %reg1			# Check if we are at Z already
	jnzr loop				
	jr setup
