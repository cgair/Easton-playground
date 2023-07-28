#include <stdio.h>
#include <stdlib.h>

/*
EVERY ADDRESS YOU SEE IS VIRTUAL
In fact, any address you can see as a programmer of a user-level program is a virtual address.
*/
int main() {
    printf("location of code: %p\n", main);
    printf("location of heap: %p\n", malloc(100e6));

    int x = 3;
    printf("location of stack: %p\n", &x);
    return 0;
}   // All of these addresses are virtual, and will be translated by the OS 
    // and hardware in order to fetch values from their true physical locations.