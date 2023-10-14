import pandas as pd
import numpy as np
from time import time
from random import sample
from sorts import counting_sort, bucket_sort

def test_sort(sorts, sizes, k, results_path, epochs=5):
    sizes = np.array(sizes)
    ks = np.empty_like(sizes)
    ks[:] = k

    frame = pd.DataFrame()
    frame["Size"] = sizes
    frame["k"] = ks

    times_correct = 0
    times_incorrect = 0

    for (sort, name) in sorts:
        print(f"Testing {name}...")
        times = np.empty_like(sizes, dtype=np.float64)

        for i, (size, k) in enumerate(zip(sizes, ks)):
            print(f"For size {size} and k {k}...")
            avg_time = 0
            for j in range(epochs):
                print(f"\tTake {j}")
                print("\tGenerating a...")
                a = sample(range(k), size)

                print("\tSorting...")
                # Pass in a copy of a so that it does not get modified
                start = time()
                b = sort(a[:])
                end = time()
                b = list(b)
                dt = end - start

                print("\tChecking answer...")
                a.sort()
                correct = a == b
                times_correct += correct
                times_incorrect += not correct

                print(f"\tcorrect: {correct}")
                if not correct:
                    idxs = np.argwhere(a != b)
                    print(idxs)
                print(f"\tTime: {dt}")
                print()
                avg_time += dt
            avg_time /= epochs
            times[i] = avg_time
            print("===============================")
            print()
        frame[name] = times

    print(f"Total sorts: {times_correct + times_incorrect}")
    print(f"Times correct: {times_correct}")
    print(f"Times incorrect: {times_incorrect}")
    frame.to_csv(results_path)        

if __name__ == "__main__":
    from radix import radix_sort
    sorts = [
        (radix_sort, "Radix Sort"),
        (counting_sort, "Counting Sort"),
        (bucket_sort, "Bucket Sort"),
    ]
    sizes = list(range(100_000, 1_000_001, 100_000))
    k = 1_000_000
    test_sort(sorts, sizes, k, "results.csv", epochs=1)
