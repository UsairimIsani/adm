#include <conio.h>

#include <iostream>

using namespace std;

template <typename T>
class Node {
   public:
    T data;
    Node<T> *next;
};

// Interface
template <typename T>
class SingleLinkedList {
   private:
    Node<T> *head;
    Node<T> *tail;
    int size;
    void empty_check();
    void bounds_check(int);
    void insert_check(int);

   public:
    SingleLinkedList();
    int get_size() const { return size; }
    bool is_empty() const { return size == 0 ? true : false; }
    T const &value_at(int);
    void push_front(const T &);
    T pop_front();
    void push_back(const T &);
    T pop_back();
    T const &front();
    T const &back();
    void insert(int, const T &);
    void erase(int);
    T const &value_from_end(int);
    void reverse();
    void remove_value(const T &);
    void display();
    ~SingleLinkedList();
};

// Implementation

template <typename T>
SingleLinkedList<T>::SingleLinkedList() : head(NULL), tail(NULL), size(0) {}

template <typename T>
T const &SingleLinkedList<T>::value_at(int index) {
    empty_check();
    bounds_check(index);
    Node<T> *current = head;
    int i = 0;
    while (i++ != index) current = current->next;
    return current->data;
}

template <typename T>
void SingleLinkedList<T>::push_front(const T &item) {
    Node<T> *new_node = new Node<T>;
    new_node->data = item;
    new_node->next = head;
    if (!tail) tail = new_node;
    head = new_node;
    size++;
}

template <typename T>
T SingleLinkedList<T>::pop_front() {
    empty_check();
    Node<T> *temp = head;
    T data = temp->data;
    head = head->next;
    if (!head) tail = head;
    delete[] temp;
    size--;
    return data;
}

template <typename T>
void SingleLinkedList<T>::push_back(const T &item) {
    Node<T> *new_node = new Node<T>;
    new_node->data = item;
    new_node->next = NULL;
    if (tail)
        tail->next = new_node;
    else
        head = new_node;
    tail = new_node;
    size++;
}

template <typename T>
T SingleLinkedList<T>::pop_back() {
    empty_check();
    Node<T> *current = head;
    Node<T> *prev = NULL;
    while (current->next) {
        prev = current;
        current = current->next;
    }
    T data = current->data;
    if (!prev)
        head = tail = NULL;
    else {
        prev->next = current->next;
        tail = prev;
    }
    delete[] current;
    size--;
    return data;
}

template <typename T>
const T &SingleLinkedList<T>::front() {
    empty_check();
    return head->data;
}

template <typename T>
T const &SingleLinkedList<T>::back() {
    empty_check();
    return tail->data;
}

template <typename T>
void SingleLinkedList<T>::insert(int index, const T &item) {
    insert_check(index);
    Node<T> *current = head;
    Node<T> *prev = NULL;
    int i = 0;
    while (i++ != index) {
        prev = current;
        current = current->next;
    }
    Node<T> *new_node = new Node<T>;
    new_node->data = item;
    new_node->next = current;
    if (!prev && !index) {
        head = new_node;
        tail = new_node;
    } else
        prev->next = new_node;
    size++;
}

template <typename T>
void SingleLinkedList<T>::erase(int index) {
    empty_check();
    bounds_check(index);

    Node<T> *current = head;
    Node<T> *prev = NULL;
    int i = 0;
    while (i++ != index) {
        prev = current;
        current = current->next;
    }
    if (!prev && !index)
        head = current->next;
    else
        prev->next = current->next;
    if (!head) tail = head;
    if (index == size - 1) tail = prev;
    delete[] current;
    size--;
}

template <typename T>
T const &SingleLinkedList<T>::value_from_end(int position) {
    empty_check();
    int index = size - position;
    bounds_check(index);
    Node<T> *current = head;
    int i = 0;
    while (i++ != index) current = current->next;
    return current->data;
}

template <typename T>
void SingleLinkedList<T>::reverse() {
    Node<T> *current = head;
    Node<T> *prev = NULL;
    Node<T> *next = NULL;
    tail = head;
    while (current) {
        next = current->next;
        current->next = prev;
        prev = current;
        current = next;
    }
    head = prev;
}

template <typename T>
void SingleLinkedList<T>::remove_value(const T &value) {
    empty_check();
    Node<T> *current = head;
    Node<T> *prev = NULL;
    bool found = false;
    while (current) {
        if (current->data == value) {
            found = true;
            break;
        }
        prev = current;
        current = current->next;
    }
    if (found) {
        if (!prev)
            head = current->next;
        else
            prev->next = current->next;
        if (!head) tail = head;
        if (tail == current) tail = prev;
        delete[] current;
        size--;
    } else
        cout << value << " not found." << endl;
}

template <typename T>
void SingleLinkedList<T>::display() {
    if (is_empty()) {
        cout << "List is empty." << endl;
        return;
    }
    Node<T> *current = head;
    while (current) {
        cout << current->data << ' ';
        current = current->next;
    }
    cout << endl;
}

template <typename T>
void SingleLinkedList<T>::empty_check() {
    if (is_empty()) {
        cout << "Error: List is Empty." << endl;
        exit(0);
    }
}

template <typename T>
void SingleLinkedList<T>::bounds_check(int index) {
    if (index < 0 || index >= size) {
        cout << "Error: Index out of Bounds." << endl;
        exit(0);
    }
}

template <typename T>
void SingleLinkedList<T>::insert_check(int index) {
    if (!size) {
        if (index != 0) {
            cout << "Error: Invalid index, should be 0." << endl;
            exit(0);
        }
    } else
        bounds_check(index - 1);
}

template <typename T>
SingleLinkedList<T>::~SingleLinkedList() {
    while (!is_empty()) pop_front();
}

// Driver Program
int main() {
    SingleLinkedList<int> L;
    L.push_front(75);
    L.pop_front();
    L.insert(0, 75);
    L.insert(1, 44);
    cout << L.pop_back() << endl;
    cout << L.pop_front() << endl;
    L.display();
    L.push_front(30);
    cout << L.front() << endl;
    cout << L.back() << endl;
    L.remove_value(45);
    L.remove_value(30);
    L.display();
    L.push_front(44);
    L.push_front(75);
    L.push_back(44);
    L.display();
    L.insert(1, 45);
    L.display();
    L.remove_value(45);
    L.display();
    L.erase(2);
    L.display();
    L.insert(1, 30);
    L.display();
    L.push_back(44);
    L.display();
    cout << L.value_from_end(3) << endl;
    cout << L.value_from_end(1) << endl;
    cout << L.value_from_end(4) << endl;
    L.reverse();
    L.display();
    cout << L.value_at(2) << endl;

    getch();
    return 0;
}