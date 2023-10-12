import numpy as np
from typing import Tuple, Any

total_comparisons = 0

def minmax(arr: np.ndarray) -> Tuple[Any, Any]:
    global total_comparisons
    if len(arr) <= 2:
        if len(arr) == 0:
            raise ValueError("Invalid Array Length")
        elif len(arr) == 1:
            return arr[0], arr[0]
        else:
            total_comparisons += 1
            return (arr[0], arr[1]) if arr[0] < arr[1] else (arr[1], arr[0])
    split = len(arr) // 2
    
    left, right = arr[:split], arr[split:]
    lmin, lmax = minmax(left)
    rmin, rmax = minmax(right)

    total_comparisons += 2
    min = lmin if lmin < rmin else rmin
    max = lmax if lmax > rmax else rmax
    
    return min, max


def _test():
    # arr = np.array([0, 3, 1, 9, -4, 4, 11, -1])
    arr = np.random.randint(-1000, 1000, size=100000)
    emin, emax = arr.min(), arr.max()
    amin, amax = minmax(arr)

    print(f"Actual   amin: {amin}, amax: {amax}")
    print(f"Expected emin: {emin}, emax: {emax}")
    print(f"total_comparisons: {total_comparisons}")

if __name__ == "__main__":
    _test()
