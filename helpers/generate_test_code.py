import sys
import os

# Assemble the supplied assembly code and print Rust code 
# to be used for easy testing purposes

# Args: arg1: assembly file name
#       arg2: array name

def main():
    os.chdir(sys.path[0])

    assembly_filepath = "assembly/" + sys.argv[1] + ".s"
    hex_filepath = "hex/" + sys.argv[1] + ".hex"

    assembly_infile = open(assembly_filepath, 'r')

    # Print the assembly code as a comment
    print("/*")
    print(assembly_infile.read())
    print("*/")

    # Call masm and generate the hex file
    if(os.system("masm -o " + hex_filepath + " " + assembly_filepath) != 0):
        print("masm failed!")
        exit(1)

    with open(hex_filepath, 'r') as infile:
        data = infile.read()

    words = data.removeprefix("v3.0 hex words plain\n").split()

    n_words = len(words)

    array_name = sys.argv[2] if len(sys.argv) == 3 else "PMEM"

    # Array header
    print("const " + array_name + ": [u32; " + str(n_words) + "] = [ ", end="")
    i = 0
    n = 0
    for word in words:
        line_end = "\n" if i == 3 else ""
        i = (i % 4) + 1
        n += 1
        print("0x" + word + "u32, ", end = line_end)

    print("];")

if __name__ == "__main__":
    main()
