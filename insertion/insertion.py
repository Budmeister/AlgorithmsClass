

def rIS(a, n):
    if n == 0:
        return
    rIS(a, n-1)
    val = a[n]
    while n > 0 and a[n-1] >= val:
        a[n] = a[n-1]
        n -= 1
    a[n] = val

def main():
    list = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
    print("Unsorted list:", list)
    rIS(list, len(list) - 1)
    print("Sorted list:", list)

if __name__ == "__main__":
    main()

