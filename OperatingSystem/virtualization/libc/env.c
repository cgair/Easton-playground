#include <stdio.h>

int main() {
    extern char **environ;  // 当 reset 状态机时, environ 有值吗, 
                            // 若没有, 是谁赋值给它的?
                            // (gdb) watch (char**)environ
    for (char **env = environ; *env; env ++) {
        printf("%s\n", *env);
    }
}

/*
 environ
+--------+
|    .---|---+
+--------+   |
             |    +--------+     +--------+
             +--->|  char*-|---->|  USER  |
                  +--------+     +--------+
                  |  char* |
                  +--------+
                  |  char* |
                  +--------+
                  |   ...  |
                  +--------+

*/

