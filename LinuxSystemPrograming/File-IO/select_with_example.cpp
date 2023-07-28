#include <sys/types.h>
#include <sys/time.h>
#include <unistd.h>
#include <iostream>


#define TIMEOUT 5 /* select timeout in seconds */
#define BUF_LEN 1024 /* read buffer in bytes */


int main()
{   
    struct timeval tv;
    fd_set readfds;
    int ret;

    /* Wait on stdin for input. */
    // 集合中的 fds 并不直接操作, 使用辅助宏
    FD_ZERO(&readfds); // 从指定集合移除所有 fds, 每次调用 select 需要调用该宏
    FD_SET(STDIN_FILENO, &readfds); /* add 'fd' to the set */

    /* Wait up to five seconds. */
    tv.tv_sec = TIMEOUT;
    tv.tv_usec = 0;

    /* All right, now block! */
    ret = select(
        STDIN_FILENO + 1, // 注意❗️
        &readfds,
        nullptr,
        nullptr,
        &tv
    );
    if (ret == -1) {
        std::cerr << "select\n";
        return 1;
    } else if (!ret) {
        std::cout << TIMEOUT << "seconds elapsed.\n";
        return 0;
    }
    /*
    * Is our file descriptor ready to read?
    * (It must be, as it was the only fd that
    * we provided and the call returned
    * nonzero, but we will humor ourselves.)
    */
    if (FD_ISSET(STDIN_FILENO, &readfds)) {
        char buf[BUF_LEN];
        int len;
        /* guaranteed to not block */
        len = read(STDIN_FILENO, buf, BUF_LEN);
        if (len == -1) {
            std::cerr << "select\n";
            return 1;
        }
        if (len) {
            buf[len] = '\0';
            std::cout << "read: " << buf << std::endl;
        }
    }

    return 0;
}