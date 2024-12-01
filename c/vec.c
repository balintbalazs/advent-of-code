#include <stdio.h>
#include "vec.h"
// typedef struct Vec;

#define OK 0
#define ERR 1



Vec create_vec(int capacity) {
    int* head = malloc(capacity * sizeof(int));

    Vec vec;
    vec.head = head;
    vec.capacity = capacity;
    vec.size = 0;

    return vec;
}

int double_vec_capacity(Vec* vector) {
    vector->capacity *= 2;
    vector->head = realloc(vector->head, vector->capacity * sizeof(int));

    if (vector->head == NULL) {
        perror("Couldn't double vector capacity\n");
        return ERR;
    }

    return OK;
}

int push_vec(Vec* vector, int element) {
    if (vector->size == vector->capacity) {
        int ret = double_vec_capacity(vector);
        if (ret == ERR) {
            perror("Couldn't push to vector\n");
            return ERR;
        }
    }
    vector->head[vector->size] = element;
    vector->size += 1;
    return OK;
}

void free_vec(Vec vector) {
    free(vector.head);
}