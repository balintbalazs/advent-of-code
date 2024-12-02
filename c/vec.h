#ifndef VEC_H
#define VEC_H

#include <stdio.h>
#include <stdlib.h>

typedef struct
{
    int *head;
    int capacity;
    int size;
} Vec;

#define OK 0
#define ERR 1

Vec create_vec(int capacity)
{
    int *head = calloc(capacity, sizeof(int));

    Vec vec;
    vec.head = head;
    vec.capacity = capacity;
    vec.size = 0;

    return vec;
}

int double_vec_capacity(Vec *vector)
{
    vector->capacity *= 2;
    vector->head = realloc(vector->head, vector->capacity * sizeof(int));

    if (vector->head == NULL)
    {
        perror("Couldn't double vector capacity\n");
        return ERR;
    }

    return OK;
}

int push_vec(Vec *vector, int element)
{
    if (vector->size == vector->capacity)
    {
        int ret = double_vec_capacity(vector);
        if (ret == ERR)
        {
            perror("Couldn't push to vector\n");
            return ERR;
        }
    }
    vector->head[vector->size] = element;
    vector->size += 1;
    return OK;
}

void free_vec(Vec vector)
{
    free(vector.head);
}

// from https://en.cppreference.com/w/c/algorithm/qsort
int compare_ints(const void *a, const void *b)
{
    int arg1 = *(const int *)a;
    int arg2 = *(const int *)b;

    if (arg1 < arg2)
        return -1;
    if (arg1 > arg2)
        return 1;
    return 0;

    // return (arg1 > arg2) - (arg1 < arg2); // possible shortcut

    // return arg1 - arg2; // erroneous shortcut: undefined behavior in case of
    // integer overflow, such as with INT_MIN here
}

void sort_vec(Vec *vector)
{
    qsort(vector->head, vector->size, sizeof(int), compare_ints);
}

#endif