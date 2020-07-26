import unittest


def binary_search(arr, key, low, high):
    if low > high:
        return None
    mid = (low + high) // 2
    if arr[mid] == key:
        return mid
    if arr[mid] > key:
        return binary_search(arr, key, low, mid-1)
    return binary_search(arr, key, mid+1, high)


class TestBinarySearch(unittest.TestCase):
    def test_existing(self):
        self.assertEqual(1, binary_search([3, 5, 6, 7], 5, 0, 3))

    def test_non_existent(self):
        self.assertEqual(None, binary_search([3, 2, 4, 12], 8, 0, 3))


if __name__ == '__main__':
    unittest.main()
