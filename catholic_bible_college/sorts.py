import math

# assumes whole integers
def counting_sort(array):
    # get max range value
    maximum = max(array)
    n = len(array)
    # temporary working storage array
    count_array = [0] * (maximum + 1)
    # sorted output array
    sorted_array = [None] * n
    # set count[i] to number of elements equal to i
    for i in range(0, n):
        count_array[array[i]] = count_array[array[i]] + 1
    # set count[i] to number of elements less than or equal to i
    for i in range(1, maximum + 1):
        count_array[i] = count_array[i] + count_array[i-1]
    # transfer these elements to their correct positions in the sorted array
    for i in range (n - 1, -1, -1):
        sorted_array[count_array[array[i]] - 1] = array[i]
        count_array[array[i]] = count_array[array[i]] - 1
    return sorted_array



def insertion_sort(array):
    n = len(array)
    for j in range (1, n):
        key = array[j]
        i = j - 1
        while i >= 0 and array[i] > key:
            array[i+1] = array[i]
            i = i - 1
        array[i+1] = key
    return array

# assumes values are greater than or equal to 0 and less than 1
def bucket_sort(array):
    n = len(array)
    maximum_divisor = max(array) + 1
    buckets = []
    ## create each bucket as an empty list
    for i in range(0,maximum_divisor):
        buckets.append([])
    ## place the element into its respective bucket
    for i in range(0, n):
        buckets[int(array[i])].append(array[i])
    ## use insertion sort on each bucket
    for i in range(0, n):
        buckets[i] = insertion_sort(buckets[i])
    array = []
    ## concatenate all of the buckets in order
    for i in range(0,n):
        array = array + buckets[i]
    return array

if __name__ == "__main__":
    # Test input
    bucket_array = [.4, .5, .6, .1,.2,.3, .76, .75, .9]
    count_array = [5, 5, 7, 6, 4, 32, 6, 10, 11]

    print(bucket_sort(count_array))
    print(counting_sort(count_array))
