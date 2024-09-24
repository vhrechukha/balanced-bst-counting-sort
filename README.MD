
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

