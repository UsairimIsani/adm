#include <stdio.h>
int binary_search_upper(int s[], int key, int low, int high)
{
    int middle;
    /* index of middle element */
    if (low > high)
        return (low);
    middle = (low + high) / 2;
    // if (s[middle] == key)
    // return (middle);
    if (s[middle] > key)
        return (binary_search_upper(s, key, low, middle - 1));
    else
        return (binary_search_upper(s, key, middle + 1, high));
}
int binary_search_lower(int s[], int key, int low, int high)
{
    int middle;
    /* index of middle element */
    if (low > high)
        return (low);
    middle = (low + high) / 2;
    // if (s[middle] == key)
    // return (middle);
    if (s[middle] < key)
        return (binary_search_lower(s, key, middle + 1, high));
    else
        return (binary_search_lower(s, key, low, middle - 1));
}

int main()
{
    int arr[9] = {1, 2, 2, 2, 3, 4, 5, 6, 7};
    int res_lower = binary_search_lower(arr, 2, 0, 8);
    int res_upper = binary_search_upper(arr, 2, 0, 8);
    printf("%d %d", res_lower, res_upper);
}