#define NULL 0
#include <stdio.h>
#include <stdlib.h>

typedef struct list
{
    int len;
    int val;
    struct list *next;
} list;

void push_back(list *l, int val)
{
    list *temp = l;
    while (temp->next != NULL)
    {
        printf("%d ", l->val);
        temp = temp->next;
    }

    l->next = NULL;
    l->val = val;
}
void push_front(list **l, int val)
{
    list *p; /* temporary pointer */

    p = malloc(sizeof(list));
    p->val = val;
    p->next = *l;
    *l = p;
}

void print_list(list *l)
{
    while (l != NULL)
    {
        printf("%d ", l->val);
        l = l->next;
    }

    printf("\n");
}
// void initialize(list *l)
// {
//     l = malloc(sizeof(list));
//     l->len = 0;
//     l->next = NULL;
//     l->val = 0;
// }
list *init_list()
{
    return (NULL);
}
int main()
{
    list *l;
    l = init_list();

    push_front(&l, 4);
    push_back(l, 4);
    print_list(l);
}