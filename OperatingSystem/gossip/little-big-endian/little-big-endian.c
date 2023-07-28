/*
How to interpret the big and little endian storage mode of a system?

int a = 0x12345678; a 首地址为 0x200
big-endian: 高位字节数据存储在低地址

 0x200  0x201  0x202   0x203
+------+------+------+------+
| 0x12 | 0x34 | 0x56 | 0x78 |
+------+------+------+------+
*/
#include <stdio.h>

int main() 
{
    check_cpu_mode1();
    
    return 0;
}

void check_cpu_mode1()
{
    int value = 0x12345678;
    char* p = (char *)&value;
    if(*p == 0x12) printf("Big endian: %p, %x\n", p, *p);   // if(p[0] == 0x12)
    else if (*p == 0x78) {
        printf("Little endian: %p, %x\n", p, *p);
        printf("               %p, %x\n", &p[1], p[1]);
        printf("               %p, %x\n", &p[2], p[2]);
        printf("               %p, %x\n", &p[3], p[3]);
        printf("               %p, %x\n", p, *p);
    }
    else printf("Uncertain.\n");  
}
/*
p++, p--: 指针 p 移动一个**单位**, 单位大小要看 p 的数据类型. sizeof(T)
如 p 是一个 char 型, 则p++移动一个字节, 如果 int *p, 则 p++ 移动4个字节.

C 编译器认为 * 和 ++ 是同优先级操作符, 且都是从右至左结合的.
所以 *p++ 中的 ++ 只作用在 p 上, 和 *(p++)意思一样;
在(*p)++中, 由于 () 的优先级比 * 和 ++ 都高, 所以 ++ 作用在 () 内的表达式 *p 上.

*/