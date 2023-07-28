#include <stdio.h>
// #include <stdlib.h>
// #include <unistd.h>

int main(int argc, char *argv[], char *envp[]) {
    // FILE * 背后其实是一个文件描述符
    FILE * fp = fopen("a.txt", "w");
    fprintf(fp, "Hello, World");
}   // gdb 调试它 p *fp 看 _fileno
    // p *stdin (_fileno = 0)