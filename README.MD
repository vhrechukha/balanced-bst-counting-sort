
## Start

`cargo run`

## Implement class for Balanced Binary Search Tree that can insert, find and delete elements.
### Results: 
```
Tree after inserting 10:
L-- 10

Tree after inserting 20:
L-- 10
    R-- 20

Tree after inserting 30:
L-- 20
    L-- 10
    R-- 30

Tree after inserting 40:
L-- 20
    L-- 10
    R-- 30
        R-- 40

Tree after inserting 50:
L-- 20
    L-- 10
    R-- 40
        L-- 30
        R-- 50

Tree after inserting 25:
L-- 30
    L-- 20
        L-- 10
        R-- 25
    R-- 40
        R-- 50

Tree after deleting 50:
L-- 30
    L-- 20
        L-- 10
        R-- 25
    R-- 40

Tree after deleting 40:
L-- 20
    L-- 10
    R-- 30
        L-- 25

Tree after deleting 20:
L-- 25
    L-- 10
    R-- 30

Tree after deleting 10:
L-- 25
    R-- 30

Tree after deleting 30:
L-- 25

Tree after deleting 25:
```

## Generate 100 random datasets and measure complexity
### Results
```
Time taken to insert 100 elements: 836.959µs
Time taken to find 100 elements: 5.625µs
Time taken to delete 100 elements: 1.059125ms
```

#### Insertion:
The total time to insert 100 elements is approximately 837 microseconds. The time complexity of each insertion is O(log n).

#### Find:
Searching for 100 elements took only 5.625 microseconds. The search operation takes O(log n) time.

#### Deletion:
Deletion took approximately 1.059 milliseconds, which is slightly higher than the insertion time. Like insertion, the time complexity for deletion is also O(log n).

## Implement Counting Sort algorithm
### Results
```
Original array: [4, 2, 2, 8, 3, 3, 1, 9, 0, 5, 5, 7, 6, 6, 6, 10]
Sorted array: [0, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6, 6, 7, 8, 9, 10]
```

## Figure out when Counting Sort doesn’t perform
### Large Range Relative to Input Size:
If the range of numbers (k) is much larger than the number of elements (n), Counting Sort uses excessive space and time.

**Example:** Sorting 10 numbers in the range [1, 1,000,000] will require a count array of size 1,000,000 for just 10 elements.

### Non-Integer or Floating Point Numbers:
Designed for integers because it uses array indexing. Non-integer numbers like floating points can't be indexed directly.

**Example:** Sorting floating-point numbers like [1.5, 3.7, 2.2, 4.8] requires converting them to integers, which adds complexity and reduces efficiency.

### Negative Numbers with Large Range:
Can handle negative numbers by adjusting the range, a large spread between negative and positive numbers (e.g., [-1,000,000, 1,000,000]) can result in high memory usage.

**Example:** Sorting [100, -999999, 50, -1, 999999] requires an array size of 2,000,000.

### Input is Already Sorted:
Doesn't leverage already sorted data, unlike algorithms like Quick Sort or Merge Sort that have better performance on nearly sorted data.

**Example:** Sorting [1, 2, 3, 4, 5] with Counting Sort takes the same time as sorting a random array.
