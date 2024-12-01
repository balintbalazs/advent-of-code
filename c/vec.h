#ifndef VEC_H
#define VEC_H

typedef struct{
    int* head;
    int capacity;
    int size;
} Vec;

Vec create_vec(int capacity);
int push_vec(Vec* vector, int element);
void free_vec(Vec vector);

#endif