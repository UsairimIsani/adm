#include <conio.h>

#include <iostream>

using namespace std;

template <typename T>
class Node {
   public:
    T data;
    Node<T> *next;
    Node<T> *prev;
};

// Interface

template <typename T>
class DoubleLinkedList {
   private:
    Node<T> *head_sentinel;
    Node<T> *tail_sentinel;
    int size;
    void add(Node<T> *, const T &);
    void remove(Node<T> *);
    void empty_check();

   public:
    DoubleLinkedList();
    ~DoubleLinkedList();
    bool empty() const;
    const T &front() const;
    const T &back() const;
    void addFront(const T &);
    void addBack(const T &);
    void removeFront();
    void removeBack();
    void reverse();
    void display();
};

// Implementation

template <class T>
DoubleLinkedList<T>::DoubleLinkedList() {
    head_sentinel = new Node<T>;
    tail_sentinel = new Node<T>;
    head_sentinel->next = tail_sentinel;
    head_sentinel->prev = NULL;
    tail_sentinel->prev = head_sentinel;
    tail_sentinel->next = NULL;
    size = 0;
}

template <class T>
DoubleLinkedList<T>::~DoubleLinkedList() {
    while (!empty()) removeFront();
    delete[] head_sentinel;
    delete[] tail_sentinel;
}

template <class T>
bool DoubleLinkedList<T>::empty() const {
    return (head_sentinel->next == tail_sentinel);
}

template <class T>
const T &DoubleLinkedList<T>::front() const {
    empty_check();
    return head_sentinel->next->data;
}

template <class T>
const T &DoubleLinkedList<T>::back() const {
    empty_check();
    return tail_sentinel->prev->data;
}

template <class T>
void DoubleLinkedList<T>::add(Node<T> *current, const T &item) {
    Node<T> *new_node = new Node<T>;
    new_node->data = item;
    new_node->next = current;
    new_node->prev = current->prev;
    current->prev->next = current->prev = new_node;
    size++;
}

template <class T>
void DoubleLinkedList<T>::addFront(const T &item) {
    return add(head_sentinel->next, item);
}

template <class T>
void DoubleLinkedList<T>::addBack(const T &item) {
    return add(tail_sentinel, item);
}

template <class T>
void DoubleLinkedList<T>::remove(Node<T> *target) {
    target->next->prev = target->prev;
    target->prev->next = target->next;
    delete[] target;
    size--;
}

template <class T>
void DoubleLinkedList<T>::removeFront() {
    empty_check();
    return remove(head_sentinel->next);
}

template <class T>
void DoubleLinkedList<T>::removeBack() {
    empty_check();
    return remove(tail_sentinel->prev);
}

template <class T>
void DoubleLinkedList<T>::reverse() {
    DoubleLinkedList<T> *temp;
    while (!this->empty()) {
        T item = this->front();
        this->removeFront();
        temp->addFront(item);
    }
    while (!temp.empty()) {
        T item = temp->front();
        temp->removeFront();
        this->addBack(item);
    }
}

template <class T>
void DoubleLinkedList<T>::empty_check() {
    if (empty()) {
        cout << "Error: List is Empty." << endl;
        exit(0);
    }
}

template <typename T>
void DoubleLinkedList<T>::display() {
    if (empty()) {
        cout << "List is empty." << endl;
        return;
    }
    Node<T> *current = head_sentinel->next;
    while (current->next) {
        cout << current->data << ' ';
        current = current->next;
    }
    cout << endl;
}

int main() {
    DoubleLinkedList<int> db;
    db.addFront(75);
    db.addFront(44);
    db.addBack(30);
    db.display();
    db.addBack(45);
    db.removeFront();
    db.removeBack();
    db.display();
    getch();
    return 0;
}