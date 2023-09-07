from random import randint, shuffle

def gen_test_case_rand(size):
    max_num = size * 10
    retval = [randint(0, max_num) for _ in range(size)]
    return retval

def gen_test_case_sorted(size):
    retval = gen_test_case_rand(size)
    retval.sort()
    return retval

def gen_test_case_almost(size):
    retval = gen_test_case_sorted(size)
    num_swaps = size
    for _ in range(num_swaps):
        to_swap = randint(0, size - 2)
        retval[to_swap], retval[to_swap + 1] = retval[to_swap + 1], retval[to_swap]

    return retval

def gen_test_case_sorted_rev(size):
    retval = gen_test_case_sorted(size)
    retval.reverse()
    return retval

def gen_test_case_almost_rev(size):
    retval = gen_test_case_almost(size)
    retval.reverse()
    return retval

def gen_test_case_rand_10(size):
    orig = gen_test_case_rand(size // 10)
    retval = []
    for _ in range(10):
        retval += orig
    shuffle(retval)
    return retval

def save_case(to_sort, file):
    file.write(f"{len(to_sort)} ")
    file.write(" ".join([str(x) for x in to_sort]))
    file.write("\n")


kindss = [
    ("Random", gen_test_case_rand),
    ("Already sorted", gen_test_case_sorted),
    ("Almost sorted", gen_test_case_almost),
    ("Sorted in reverse", gen_test_case_sorted_rev),
    ("Almost sorted in reverse", gen_test_case_almost_rev),
    ("Random with 10 copies of each entry", gen_test_case_rand_10),
]

def main():
    for i, (name, _) in enumerate(kindss):
        print(f"[{i}] {name}")

    kinds = [int(x) for x in input("What kind of test cases would you like? ").split()]

    kinds = [kindss[x] for x in kinds]

    sizes = [int(x) for x in input("What size? ").split()]

    repeat = int(input("How many times to repeat each test? "))
    
    with open("4/test_cases.txt", "w") as file:
        for (_, kind) in kinds:
            for size in sizes:
                for _ in range(repeat):
                    to_sort = kind(size)
                    save_case(to_sort, file)


if __name__ == "__main__":
    main()
