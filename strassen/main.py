import numpy as np

def strassen(a: np.ndarray, b: np.ndarray) -> np.ndarray:
    n = a.shape[0]
    if a.shape != (n, n) or b.shape != (n, n):
        raise ValueError("Must be square matrices with the same size")
    if n == 1:
        return a * b

    mid = n // 2
    a11, a12, a21, a22 = a[:mid, :mid], a[:mid, mid:], a[mid:, :mid], a[mid:, mid:]
    b11, b12, b21, b22 = b[:mid, :mid], b[:mid, mid:], b[mid:, :mid], b[mid:, mid:]

    s1 = b12 - b22
    s2 = a11 + a12
    s3 = a21 + a22
    s4 = b21 - b11
    s5 = a11 + a22
    s6 = b11 + b22
    s7 = a12 - a22
    s8 = b21 + b22
    s9 = a11 - a21
    s10 = b11 + b12

    p1 = strassen(a11, s1)
    p2 = strassen(s2, b22)
    p3 = strassen(s3, b11)
    p4 = strassen(a22, s4)
    p5 = strassen(s5, s6)
    p6 = strassen(s7, s8)
    p7 = strassen(s9, s10)

    c11 = p5 + p4 - p2 + p6
    c12 = p1 + p2
    c21 = p3 + p4
    c22 = p5 + p1 - p3 - p7

    c = np.empty_like(a)
    c[:mid, :mid], c[:mid, mid:], c[mid:, :mid], c[mid:, mid:] = c11, c12, c21, c22
    return c

def main():
    n = 128
    shape = (n, n)
    a = np.random.uniform(size=shape)
    b = np.random.uniform(size=shape)
    print("Calculating using numpy")
    c_real = a @ b
    print("Calculating using Strassen")
    c = strassen(a, b)

    if np.all(np.abs(c - c_real) < 0.01):
        print("Strassen algorithm correct!")
    else:
        print("Strassen algorithm incorrect")

if __name__ == "__main__":
    main()