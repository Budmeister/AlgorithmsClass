from random import randint

MOVE_ONTO = "move ${a} onto ${b}"
MOVE_OVER = "move ${a} over ${b}"
PILE_ONTO = "pile ${a} onto ${b}"
PILE_OVER = "pile ${a} over ${b}"

instructions = [
    MOVE_ONTO,
    MOVE_OVER,
    PILE_ONTO,
    PILE_OVER,
]

def format_instr(instr, a, b):
    return instr.replace("${a}", str(a)).replace("${b}", str(b))

def generate_random_instruction(n):
    instr = instructions[randint(0, 3)]
    a = randint(0, n-1)
    b = randint(0, n-1)
    return format_instr(instr, a, b)

def generate_instrs(kind, n, i):
    instrs = []
    for _ in range(i):
        instr = kind(n)
        instrs.append(instr)

    return instrs

def save_instrs(n, instrs, output_dir):
    with open(output_dir, "w") as file:
        file.write(f"{n}\n")
        file.writelines((x + "\n" for x in instrs))
        file.write("quit\n")

def main():
    print("[0] Random")

    kinds = [int(x) for x in input("What kind of test cases would you like? ").split()]

    kind_namess = [
        "random"
    ]
    kindss = [
        generate_random_instruction
    ]
    kinds = [(kindss[x], kind_namess[x]) for x in kinds]

    ns = [int(x) for x in input("How big is the world? ").split()]
    is_ = [int(x) for x in input("How many instructions do you want? ").split()]
    
    for (kind, kind_name) in kinds:
        for n in ns:
            for i in is_:
                output_dir = f"test_cases/{kind_name}_{n}x{i}.txt"
                instrs = generate_instrs(kind, n, i)
                save_instrs(n, instrs, output_dir)


if __name__ == "__main__":
    main()
