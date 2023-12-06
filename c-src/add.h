#pragma once

int add(int a, int b);

struct Animal
{
    int type;
    char *name;
};

typedef struct Animal Animal;

Animal *create_animal(int type, char *name);

void free_animal(Animal *animal);