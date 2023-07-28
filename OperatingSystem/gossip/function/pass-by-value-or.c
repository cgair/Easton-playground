#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void swap1(int a, int b)
{
    int tmp;
    tmp = a;
    a = b;
    b = tmp;
    printf("[swap-1] Swapping: a = %d, b = %d\n", a, b);
}

// c 语言没有引用传
void swap2(int *a, int *b)
{
    int tmp;
    tmp = *a;
    *a = *b;
    *b = tmp;
    printf("[swap-2] Swapping: a = %d, b = %d\n", *a, *b);
}

void get_memory1(char *p)
{
    p = (char *) malloc(100);
    strcpy(p, "Hello world");
}
// 二级指针作为函数的形式参数可以让被调用函数使用其他函数的指针类型存储区
void get_memory2(char **p)
{
    *p = (char *) malloc(100);
    strcpy(*p, "Hello world");
}


int main()
{
    int a = 0, b = 1;
    printf("[main]Before swap: a = %d, b = %d\n", a, b);
    swap1(a, b);
    swap2(&a, &b);
    printf("[main] After swap: a = %d, b = %d\n", a, b);

    char *str = NULL;
    // get_memory1(str);
    get_memory2(&str);
    printf("[main]%s\n", str);  
    free(str);

    return 0;
}

