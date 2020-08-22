#include <conio.h>

#include <iostream>

using namespace std;

template <typename T>
class Node {
   private:
    T data;
    Node<T> *next;
    friend class SingleLinkedList<T>;
};

template <typename T>
class SingleLinkedList {
   private:
    Node<T> *head;
    int size;
    void empty_check();
    void bounds_check(int);
    void insert_check(int);

   public:
    SingleLinkedList();
    int get_size() const { return size; }
    bool is_empty() const { return size == 0 ? true : false; }
    T const &value_at(int) const;
    void push_front(const T &);
    T &pop_front();
    void push_back(const T &);
    T &pop_back();
    T const &front() const;
    T const &back() const;
    void insert(int, const T &);
    void erase(int);
    T const &value_from_end(int) const;
    void reverse();
    void remove_value(const T &);
    void display();
    ~SingleLinkedList();
};

template <typename T>
SingleLinkedList<T>::SingleLinkedList() : head(NULL) {}

template <typename T>
T const &SingleLinkedList<T>::value_at(int index) const {
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
    head = new_node;
    size++;
}

template <typename T>
T &SingleLinkedList<T>::pop_front() {
    empty_check();
    Node<T> *temp = head;
    T data = temp->data;
    head = head->next;
    delete[] temp;
    size--;
    return data;
}

template <typename T>
void SingleLinkedList<T>::push_back(const T &item) {
    if (!head) return push_front(item);
    Node<T> *current = head;
    while (current->next) current = current->next;
    Node<T> *new_node = new Node<T>;
    new_node->data = item;
    new_node->next = current->next;
    current->next = new_node;
    size++;
}

template <typename T>
T &SingleLinkedList<T>::pop_back() {
    empty_check();
    Node<T> *current = head;
    Node<T> *prev = NULL;
    while (current->next) {
        prev = current;
        current = current->next;
    }
    T data = current->data;
    prev->next = current->next;
    delete[] current;
    size--;
    return data;
}

template <typename T>
const T &SingleLinkedList<T>::front() const {
    empty_check();
    return head->data;
}

template <typename T>
T const &SingleLinkedList<T>::back() const {
    empty_check();
    Node<T> *current = head;
    while (current->next) current = current->next;
    return current->data;
}

template <typename T>
void SingleLinkedList<T>::insert(int index, const T &item) {
    insert_check(index);
    bounds_check(index);
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
    if (!prev && !index)
        head = new_node;
    else
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
    delete[] current;
    size--;
}

template <typename T>
T const &SingleLinkedList<T>::value_from_end(int position) const {
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
    while (current) {
        if (current->data == value) break;
        prev = current;
        current = current->next;
    }
    if (!prev)
        head = current->next;
    else
        prev->next = current->next;
    delete[] current;
    size--;
}

template <typename T>
void SingleLinkedList<T>::display() {
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
    if (!size && index != 0) {
        cout << "Error: Invalid index, should be 0." << endl;
        exit(0);
    }
}

template <typename T>
SingleLinkedList<T>::~SingleLinkedList() {
    while (!is_empty()) pop_front();
}
