#include "add.h"
#include "stdlib.h"

int add(int a, int b)
{
    return a + b;
}

Animal *create_animal(int type, char *name)
{
    Animal *animal = malloc(sizeof(Animal));
    animal->type = type;
    animal->name = name;
    return animal;
}

void free_animal(Animal *animal)
{
    free(animal->name);
    free(animal);
}