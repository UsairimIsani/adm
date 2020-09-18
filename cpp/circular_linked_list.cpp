#include <conio.h>

#include <iostream>

using namespace std;

template <typename T>
class Node {
   public:
    T data;
    Node<T>* next;
};

// Interface
template <typename T>
class CircularLinkedList {
   private:
    Node<T>* cursor;
    int size;
    void empty_check() const;

   public:
    CircularLinkedList();
    ~CircularLinkedList();
    bool empty() const;
    const T& front() const;
    const T& back() const;
    void advance();
    void add(const T&);  // add after cursor
    void remove();       // remove after cursor
    void display();
};

// Implementation

template <class T>
CircularLinkedList<T>::CircularLinkedList() : cursor(NULL), size(0) {}

template <class T>
CircularLinkedList<T>::~CircularLinkedList() {
    while (!empty()) remove();
}

template <class T>
bool CircularLinkedList<T>::empty() const {
    return (cursor == NULL);
}

template <class T>
const T& CircularLinkedList<T>::front() const {
    empty_check();
    return cursor->next->data;
}

template <class T>
const T& CircularLinkedList<T>::back() const {
    empty_check();
    return cursor->data;
}

template <class T>
void CircularLinkedList<T>::advance() {
    if (empty()) return;
    cursor = cursor->next;
}

template <class T>
void CircularLinkedList<T>::add(const T& item) {
    Node<T>* new_node = new Node<T>;
    new_node->data = item;
    if (cursor == NULL) {
        new_node->next = new_node;
        cursor = new_node;
    } else {
        new_node->next = cursor->next;
        cursor->next = new_node;
    }
    size++;
}

template <class T>
void CircularLinkedList<T>::remove() {
    Node<T>* target = cursor->next;
    if (target == cursor)
        cursor == NULL;
    else
        cursor->next = target->next;
    delete[] target;
    size--;
}

template <typename T>
void CircularLinkedList<T>::empty_check() const {
    if (empty()) {
        cout << "Error: List is Empty." << endl;
        exit(0);
    }
}

template <class T>
void CircularLinkedList<T>::display() {
    Node<T>* current = cursor->next;
    if (empty()) {
        cout << "List is empty." << endl;
        return;
    }
    do {
        cout << current->data << " ";
        current = current->next;
    } while (current != cursor->next);
    cout << endl;
}

int main() {
    CircularLinkedList<int> L;
    L.add(75);
    L.add(30);
    L.add(44);
    L.display();
    cout << L.front() << endl;
    cout << L.back() << endl;
    L.advance();
    cout << L.front() << endl;
    cout << L.back() << endl;
    L.remove();
    L.display();
    cout << L.front() << endl;
    cout << L.back() << endl;

    getch();
    return 0;
}