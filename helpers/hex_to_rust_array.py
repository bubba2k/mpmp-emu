import sys

def main():
    with open(sys.argv[1], 'r') as infile:
        data = infile.read()

    words = data.removeprefix("v3.0 hex words plain\n").split()

    print("[ ", end="")
    i = 0
    n = 0
    for word in words:
        line_end = "\n" if i == 3 else ""
        i = (i % 4) + 1
        n += 1
        print("0x" + word + "u32, ", end = line_end)

    print("] // n = " + str(n))

if __name__ == "__main__":
    main()
