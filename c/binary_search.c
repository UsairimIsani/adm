#include <stdio.h>
int binary_search(int s[], int key, int low, int high)
{
    int middle;
    /* index of middle element */
    if (low > high)
        return (-1);
    middle = (low + high) / 2;
    if (s[middle] == key)
        return (middle);
    if (s[middle] > key)
        return (binary_search(s, key, low, middle - 1));
    else
        return (binary_search(s, key, middle + 1, high));
}

int main()
{
    int arr[7] = {1, 2, 3, 4, 5, 6, 7};
    int res = binary_search(arr, 2, 0, 6);
    printf("%d", res);
}