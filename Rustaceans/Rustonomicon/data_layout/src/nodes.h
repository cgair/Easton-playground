#include <stdbool.h>
#include <stdint.h>
/*
结构字节对齐的原则主要有:
1.   数据类型自身的对齐值: char 型数据自身对齐值为 1 字节, short 型数据为 2 字节, int/float 型为 4 字节, double 型为 8 字节 ... (操作系统不同可能由偏差)
2. 结构体或类的自身对齐值: 其成员中自身对齐值最大的那个值. (结构体的每一个成员相对结构体首地址的偏移量应该是对其参数的整数倍, 如果不满足则补足前面的字节使其满足)
3.           指定对齐值：#pragma pack (value) 时的指定对齐值 value
*/

/** 
 *  @brief node1为一个空结构体
 * 在 C 中空结构体的大小为 0 字节
 * 在 C++ 中空结构体的大小为 1 字节
*/
typedef struct node1 {
} S1;

/**
 * @brief node2的内存结构
 * (4 — 1 — 1 (补) — 2), 总大小为 8 字节
 * 
 * (结构体的每一个成员相对结构体首地址的偏移量应该是对其参数的整数倍)
 */
typedef struct node2 {  
    int a;
    char b;
    short c;
} S2;

/**
 * @brief node3的内存结构
 * (1 — 3 (补) — 4 — 2 — 2 (补)), 总大小为 12 字节
 *  
 * (结构体的每一个成员相对结构体首地址的偏移量应该是对其参数的整数倍)
 */
typedef struct node3 {
    char a;
    int b;
    short c;
} S3;

/**
 * @brief node4的内存结构
 * (4 - 2 — 2 (补)), 总大小为 8 字节
 *  
 * 静态变量被分配到静态数据区, 不在 sizeof 计算的范围内
 */
typedef struct node4 {
    int a;
    short b;
    // static int c; // 静态变量单独存放在静态数据区 
    // <https://stackoverflow.com/questions/27220758/type-name-does-not-allow-storage-class-to-be-specified>
} S4;

/**
 * @brief node5的内存结构
 * (1 — 1 — 2), 总大小为 4 字节
 */
typedef struct node5 {
    bool a;
    S1 b;
    short c;
} S5;

/**
 * @brief node6的内存结构
 * (1 — 3 (补) — 8 — 4), 总大小为 16 字节
 * 
 * 注意结构体变量的对齐参数的计算
 */
typedef struct node6 {
    bool a;
    S2 b;
    int c;
} S6;

/**
 * @brief node7的内存结构
 * (1 — 3 (补) — 8 — 4 (补) — 8 — 4 — 4 (补)), 总大小为 32 字节
 */
typedef struct node7 {  // #pragma pack(n) 为 8 -> 成员中自身对齐值最大的那个值
    bool a;   // 对于 a 变量, 其对齐参数为 1, 此时 offset=0, 可以被 1 整除, 因此为其分配 1 字节空间.
    S2 b;     // 对于b变量, 其对齐参数为 4 (S2 结构体的成员变量最大对齐参数为 int -> 4),
              // 此时 offset = 1, 不能被 4 整除, 因此填充 3 字节后为其分配8字节空间.
    double d; // 对于 d 变量, 其对齐参数为 8, 此时 offset = 12, 不能被 8 整除, 因此填充 4 字节后为其分配 8 字节空间.
    int c;    // 对于 c 变量, 其对齐参数为 4, 此时 offset = 24, 可以被 4 整除, 因此为其分配 4 字节空间.
              // 此时所有变量都分配完, 但此时 offset = 28, 不能被最大对齐参数 8 整除, 因此填充 4 字节使其可以被8整除.
} S7;         // 所以最后 node7 的大小为32字节
// 

/**
 * @brief node8的内存结构
 * (1 — 3 (补) — 8 — 4), 总大小为 16 字节
 */
typedef struct node8 {
    bool a;
    S2 b;
    char* c;
} S8;
