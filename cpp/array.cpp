#include <iostream>
#include <cmath>
#include <conio.h>

using namespace std;

// Interface
template <class T>
class Vector
{
private:
    T *arr;
    int size, capacity;
    void resize(int);
    void bound_check(int);

public:
    Vector(int = 4);
    int get_size() const { return size; }
    int get_capacity() const { return capacity; }
    bool is_empty() const { return (size == 0 ? true : false); }
    int find(T) const;
    T &operator[](int);
    int push(T);
    int insert(int, T);
    int prepend(T);
    T &pop();
    int del(int);
    int remove(T);
    void display();
    ~Vector();
};

// Implementation
template <class T>
Vector<T>::Vector(int n) : capacity(n), arr(new T[n]), size(0) {}

template <class T>
void Vector<T>::resize(int new_capacity)
{
    capacity = ceil(new_capacity);
    T *new_arr = new T[capacity];
    for (int i = 0; i < size; i++)
        new_arr[i] = arr[i];
    delete[] arr;
    arr = new_arr;
}

template <class T>
void Vector<T>::bound_check(int index)
{
    if (index < 0 || index >= size)
    {
        cout << "Error: Index out of Bounds." << endl;
        exit(0);
    }
}

template <class T>
T &Vector<T>::operator[](int index)
{
    bound_check(index);
    return *(arr + index);
}

template <class T>
int Vector<T>::push(T item)
{
    if (size == capacity)
        resize(2 * capacity);
    arr[size++] = item;
    return size;
}

template <class T>
int Vector<T>::insert(int index, T item)
{
    bound_check(index);
    if (size == capacity)
        resize(2 * capacity);
    int i;
    for (i = size - 1; i >= index; i--)
        arr[i + 1] = arr[i];
    arr[i + 1] = item;
    return ++size;
}

template <class T>
int Vector<T>::prepend(T item)
{
    return insert(0, item);
}

template <class T>
int Vector<T>::find(T item) const
{
    for (int i = 0; i < size; i++)
        if (arr[i] == item)
            return i;
    return -1;
}

template <class T>
T &Vector<T>::pop()
{
    if (--size <= ceil(0.25 * capacity))
        resize(0.5 * capacity);
    return arr[size];
}

template <class T>
int Vector<T>::del(int index)
{
    bound_check(index);
    for (int i = index; i < size - 1; i++)
        arr[i] = arr[i + 1];
    if (--size <= ceil(0.25 * capacity))
        resize(0.5 * capacity);
    return size;
}

template <class T>
int Vector<T>::remove(T item)
{
    int new_size = 0;
    T *new_arr = new T[capacity];
    for (int i = 0; i < size; i++)
        if (arr[i] != item)
            new_arr[new_size++] = arr[i];
    size = new_size;
    delete[] arr;
    arr = new_arr;
    if (size <= ceil(0.25 * capacity))
        resize(0.5 * capacity);
    return size;
}

template <class T>
void Vector<T>::display()
{
    for (int i = 0; i < size; i++)
        cout << arr[i] << ' ';
    cout << endl;
}

template <class T>
Vector<T>::~Vector()
{
    delete[] arr;
}

// driver program
int main()
{
    cout << "For Integer data_type" << endl;
    Vector<int> integers;
    integers.push(44);
    integers.push(75);
    integers.push(30);
    integers.push(57);
    integers.display();
    cout << integers.get_capacity() << endl;
    cout << integers.get_size() << endl;
    integers.push(8);
    cout << integers.get_capacity() << endl;
    cout << integers.get_size() << endl;
    cout << integers.is_empty() << endl;
    cout << integers.find(75) << endl;
    cout << integers.insert(1, 30) << endl;
    cout << integers.insert(1, 54) << endl;
    integers.display();
    cout << integers.get_capacity() << endl;
    cout << integers.get_size() << endl;
    cout << integers.prepend(75) << endl;
    cout << integers.prepend(45) << endl;
    cout << integers.get_capacity() << endl;
    cout << integers.get_size() << endl;
    integers.display();
    cout << integers.pop() << endl;
    cout << integers.get_capacity() << endl;
    cout << integers.get_size() << endl;
    integers.display();
    cout << integers.remove(75) << endl;
    cout << integers.get_capacity() << endl;
    cout << integers.get_size() << endl;
    integers.display();
    cout << integers.del(2) << endl;
    cout << integers.remove(30) << endl;
    cout << integers.get_capacity() << endl;
    cout << integers.get_size() << endl;
    integers.display();

    cout << "\n\nFor char data_type" << endl;
    Vector<char> chars;
    chars.push('S');
    chars.push('M');
    chars.push('A');
    chars.push('B');
    chars.display();
    cout << chars.get_capacity() << endl;
    cout << chars.get_size() << endl;
    chars.push('m');
    cout << chars.get_capacity() << endl;
    cout << chars.get_size() << endl;
    cout << chars.is_empty() << endl;
    cout << chars.find('M') << endl;
    cout << chars.insert(1, 'A') << endl;
    cout << chars.insert(1, 'K') << endl;
    chars.display();
    cout << chars.get_capacity() << endl;
    cout << chars.get_size() << endl;
    cout << chars.prepend('M') << endl;
    cout << chars.prepend('a') << endl;
    cout << chars.get_capacity() << endl;
    cout << chars.get_size() << endl;
    chars.display();
    cout << chars.pop() << endl;
    cout << chars.get_capacity() << endl;
    cout << chars.get_size() << endl;
    chars.display();
    cout << chars.remove('M') << endl;
    cout << chars.get_capacity() << endl;
    cout << chars.get_size() << endl;
    chars.display();
    cout << chars.del(2) << endl;
    cout << chars.remove('A') << endl;
    cout << chars.get_capacity() << endl;
    cout << chars.get_size() << endl;
    chars.display();

    getch();
    return 0;
}
