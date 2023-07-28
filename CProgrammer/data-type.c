#include <stdio.h>

int main()
{
    /* int 类型 */
    int ten = 10;
    int two = 2;
    printf("Type int has a size of %zd bytes.\n", sizeof(int));     // sizeof 是 C 语言的内置运算符, 以字节为单位给出指定类型的大小

    printf("Doing it right: ");
    // %d 指明了在一行中打印整数的位置. 
    // %d称为转换说明, 它指定了 printf() 应使用什么格式来显示一个值
    printf("%d minus %d is %d\n", ten, two, ten - two);

    // 显示八进制和十六进制
    printf("                %o minus %o is %o\n", ten, two, ten - two);
    printf("                %x minus %x is %x\n", ten, two, ten - two);
    printf("                %#x minus %#x is %#x\n", ten, two, ten - two);
    printf("                %#X minus %#X is %#X\n", ten, two, ten - two);


    // printf("Doing it wrong: ");
    // printf("%d minus %d is %d\n", ten); // 遗漏2个参数

    /* 字符: char类型 */
    printf("Type char has a size of %zd bytes.\n", sizeof(char));
    char grade = 'A';
    printf("ASCII: %d, char: %c\n", grade, grade);

    /*
    char ch;
    printf("Please enter a character.\n");
    scanf("%c", &ch);
    printf("The code for %c is %d.\n", ch, ch);
    */

    // 布尔值表示 true 和 false. C语言用 1 表示 true, 0 表示false.

    return 0;
}