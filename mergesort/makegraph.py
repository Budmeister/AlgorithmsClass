from matplotlib import pyplot as plt
import numpy as np

def main():
    kindss = [
        "Random",
        "Already sorted",
        "Almost sorted",
        "Sorted in reverse",
        "Almost sorted in reverse",
        "Random with 10 copies of each entry",
    ]

    plt.suptitle("Runtime and number of comparisons\nversus input size for Mergesort")

    ax = plt.subplot(121)
    ax.set_title("Runtime versus input size")
    for i in range(6):
        data_filename = f"4/data{i}.csv"
        with open(data_filename, "r") as file:
            lines = file.readlines()
        
        data = np.array([[int(x) for x in line.split(",")] for line in lines])
        sizes, times, num_comparisons = data.T

        plt.scatter(sizes, times, label=kindss[i])
    xlim = list(plt.xlim())
    xlim[0] = 0
    ylim = list(plt.ylim())
    ylim[0] = 0
    plt.xlim(xlim)
    plt.ylim(ylim)
    plt.xlabel("Input size")
    plt.ylabel("Calculation time (ms)")
    plt.legend()

    ax = plt.subplot(122)
    ax.set_title("Number of comparisons versus input size")
    for i in range(6):
        data_filename = f"4/data{i}.csv"
        with open(data_filename, "r") as file:
            lines = file.readlines()
        
        data = np.array([[int(x) for x in line.split(",")] for line in lines])
        sizes, times, num_comparisons = data.T

        plt.scatter(sizes, num_comparisons, label=kindss[i])
    xlim = list(plt.xlim())
    xlim[0] = 0
    ylim = list(plt.ylim())
    ylim[0] = 0
    plt.xlim(xlim)
    plt.ylim(ylim)
    plt.xlabel("Input Size")
    plt.ylabel("Number of Comparisons")
    plt.legend()
    plt.show()


if __name__ == "__main__":
    main()