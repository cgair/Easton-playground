#include <string.h>
#include <stdio.h>

extern void mylib_log(char* msg, int32_t level);
extern void mylib_log_ugly(char* msg, int32_t level);

int main() 
{
    char s[] = "RUNOOB";
    printf("s = %s, len = %lu", s, strlen(s));
    mylib_log(s, 0);
    mylib_log_ugly(s, 0);


    char s[] = "RUNOOB";
    printf("s = %s, len = %lu", s, strlen(s));
    mylib_log(s, 0);
    mylib_log_ugly(s, 0);
}

/**
 * 
 * How to compile?
 * 
 * ```bash
 * gcc -arch x86_64 <path-to-libffi.dylib> -o myliblog ./myliblog.c 
 * ```
 */