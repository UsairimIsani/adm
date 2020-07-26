import unittest


def first(arr, key, low, high):
    if (low > high):
        return None
    mid = (low+high) // 2
    if (mid == 0 or arr[mid-1] < key) and arr[mid] == key:
        return mid
    if key < arr[mid]:
        return first(arr, key, low, mid-1)
    return first(arr, key, mid+1, high)


def last(arr, key, low, high, size):
    if low > high:
        return None
    mid = (low+high) // 2
    if (mid == size-1 or arr[mid+1] > key) and arr[mid] == key:
        return mid
    if key < arr[mid]:
        return last(arr, key, low, mid-1, size)
    return last(arr, key, mid+1, high, size)


def count_occurances_sorted(arr, key):
    n = len(arr)
    first_index = first(arr, key, 0, n-1)
    if not first_index:
        return 0
    last_index = last(arr, key, first_index, n-1, n)
    return (last_index-first_index+1)


class TestCountOccurenceSorted(unittest.TestCase):
    def test_occurences(self):
        arr = [1, 2, 2, 3, 3, 3, 3]
        self.assertEqual(4, count_occurances_sorted(arr, 3))
        self.assertEqual(0, count_occurances_sorted(arr, 9))


if __name__ == '__main__':
    unittest.main()
