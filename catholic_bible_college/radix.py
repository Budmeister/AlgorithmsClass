
def radix_sort(a):
    i = 0
    while True:
        bit0 = []
        bit1 = []
        # print(f"i: {i}")
        for num in a:
            bit = (num >> i) & 1
            (bit1 if bit else bit0).append(num)
            # if bit:
            #     bit1.append(num)
            # else:
            #     bit0.append(num)
        # print(f"bit0: {bit0}")
        # print(f"bit1: {bit1}")
        # print()
        a = bit0 + bit1
        if len(bit0) == 0 or len(bit1) == 0:
            break
        i += 1
    return a

def _main():
    from random import shuffle
    from time import time

    k = 10_000_000
    n = 1_000_000

    print("Generating a...")
    a = list(range(k))
    shuffle(a)
    a = a[:n]
    # print(a)
    # print()

    print("Sorting...")
    start = time()
    b = radix_sort(a)
    end = time()
    # print(b)

    print("Checking answer...")
    a.sort()
    correct = a == b

    print()
    print("==============================")
    print()

    print(f"k: {k}")
    print(f"n: {n}")
    print(f"correct: {correct}")
    print(f"Time: {end - start}s")


if __name__ == "__main__":
    _main()