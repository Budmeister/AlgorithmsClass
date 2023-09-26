from random import shuffle
import matplotlib.pyplot as plt
from tqdm import tqdm

def hiring(n):
    interviews = 0
    hires = 0

    candidates = list(range(n))
    shuffle(candidates)

    best = 0
    for c in candidates:
        interviews += 1
        if c > best:
            hires += 1
            best = c
    return interviews, hires

def test_hiring(display=False):
    # ns = [
    #     10, 20, 30, 40, 50, 60, 70, 80, 90,
    #     100, 200, 300, 400, 500, 600, 700, 800, 900,
    #     1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000
    # ]
    ns = list(range(0, 10000, 50))
    repetitions = 50
    avg_intss = []
    avg_hiress = []
    for n in tqdm(ns) if not display else ns:
        total_ints = 0
        total_hires = 0
        for _ in range(repetitions):
            ints, hires = hiring(n)
            total_ints += ints
            total_hires += hires
        avg_ints = total_ints / repetitions
        avg_hires = total_hires / repetitions
        avg_intss.append(avg_ints)
        avg_hiress.append(avg_hires)

        if display:
            print("========================")
            print(f"n: {n}")
            print(f"num trials: {repetitions}")
            print(f"avg interviews: {avg_ints}")
            print(f"avg hires: {avg_hires}")
            print("========================")
            print()
    return ns, avg_intss, avg_hiress

def plot_results(ns, avg_ints, avg_hires):
    plt.subplot(121)
    plt.plot(ns, avg_ints, label="Average Interviews")
    plt.xlabel("n")
    plt.ylabel("Number of interviews")
    plt.title(f"Average Interviews versus n")
    # plt.legend()

    plt.subplot(122)
    plt.plot(ns, avg_hires, label="Average Hires")
    plt.xlabel("n")
    plt.ylabel("Number of hires")
    plt.title(f"Average Hires versus n")
    # plt.legend()

    plt.show()


if __name__ == "__main__":
    ns, avg_ints, avg_hires = test_hiring(False)
    plot_results(ns, avg_ints, avg_hires)
