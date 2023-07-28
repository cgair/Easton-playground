// 同时检测一个 stdin 读 和一个 stdout 写是否阻塞
#include <sys/poll.h>
#include <sys/types.h>
#include <unistd.h>     // It stands for "Unix Standard" and provides access to 
                        // various basic functions and macros for operations such as 
                        // file access, process control, and others that are standard in Unix and Unix-like systems.
#include <iostream>

#define TIMEOUT 5  /* poll timeout, in seconds */

int main()
{   
    struct pollfd fds[2];
    int ret;

    /* watch stdin for input */
    fds[0].fd = STDIN_FILENO;
    fds[0].events = POLLIN; // 每个结构体的 event 字段是要监听的
                            // 文件描述符事件的一组位掩码, 用户设置

    /* watch stdout for ability to write (almost
       always true) */
    fds[1].fd = STDOUT_FILENO;
    fds[1].events = POLLOUT;

    /* All set, block! */
    ret = poll (fds, 2, TIMEOUT * 1000);
    if (ret == -1) {
        std::cerr << "poll" << std::endl;
        return 1;
    }

    if (!ret) {
        std::cout << TIMEOUT << "seconds elapsed.\n";
        return 0;
    }

    if (fds[0].revents & POLLIN) std::cout << "stdin is readable.\n";
    if (fds[1].revents & POLLOUT) std::cout << "stdout is writable.\n";

    return 0;
}
