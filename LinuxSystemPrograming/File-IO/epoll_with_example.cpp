#include <sys/epoll.h>
#include <iostream>
#include <cerrno>
#include <cstring> // for strerror
#include <unistd.h>

#define MAX_EVENTS 64
#define BUF_LEN 1024

/**
 * [Lab 1] epoll_create
*/
int labOne() {
    int epfd;
    // ‘epoll_create’ 会创建一个 epoll instance 同时返回一个引用该实例的文件描述符.
    // epoll instance 实例内部存储 (from a user-space perspective)
    // - 监听列表 (interest list): 所有要监听的文件描述符, 使用红黑树
    // - 就绪列表 (ready list): 所有就绪的文件描述符, 使用链表
    // epfd = epoll_create(100); /* plan to watch ~100 fds */
    // epfd = epoll_create(-100); /* EINVAL */
    epfd = epoll_create(2);

    if (epfd < 0) {
        std::cout << " Error number: " << errno << std::endl;
        // Do note that errno is not thread-safe. 
        // If you're working in a multithreaded environment, 
        // you might have problems if you try to use errno directly. 
        // Instead, you should use strerror_r, the thread-safe equivalent of strerror.
        std::cout << "Error message: " << strerror(errno) << std::endl;
    }

    return epfd;
}


/**
 * [Lab 2] epoll_ctl
*/
void labTwo(int epfd) {

    struct epoll_event event;
    int ret;
    event.data.fd = STDIN_FILENO;
    event.events = EPOLLIN | EPOLLOUT;
    ret = epoll_ctl(epfd, EPOLL_CTL_ADD, STDIN_FILENO, &event);
    if (ret) {
        std::cout << " Error number: " << errno << std::endl;
        std::cout << "Error message: " << strerror(errno) << std::endl;
    }
}

/**
 * [Lab 3] epoll_wait
*/
int labThree(int epfd) {
    struct epoll_event* events;
    int nr_events;

    events = (epoll_event*)malloc(sizeof(struct epoll_event) * MAX_EVENTS);
    if (!events) {
        std::cerr << "malloc\n";
        return 1;
    }

    nr_events = epoll_wait(epfd, events, MAX_EVENTS, -1);

    if (nr_events < 0) {
        std::cerr << "epoll_wait\n";
        free(events);
        return 1;
    }

    for (int i = 0; i < nr_events; ++i) {
        int fd = events[i].data.fd;
        std::cout << "event=" << events[i].events 
                  << " on fd="
                  << fd << std::endl;
        /*
        * We now can, per events[i].events, operate on
        * events[i].data.fd without blocking.
        */
        char buf[1024];
        int ret = read(fd, buf, 1024);
        std::cout << buf << std::endl;
    }

    free (events);
    close(epfd);
    return 0;
}


/**
 * [Lab 4] Level-triggered and edge-triggered
*/
int labFour() {
    int pipefd[2];
    int epfd;
    struct epoll_event event, events[10];
    // or
    // struct epoll_event* events;
    // events = (epoll_event*)malloc(sizeof(struct epoll_event) * 10);

    char buf[BUF_LEN];

    if (pipe(pipefd) == -1) { //  The first descriptor connects to the read end of the pipe; 
                              // the second connects to the write end.
        std::cerr << "pipe\n";
        exit(EXIT_FAILURE);
    }

    epfd = epoll_create1(0);
    if (epfd == -1) {
        perror("epoll_create1");
        exit(EXIT_FAILURE);
    }

    // event.events = EPOLLIN; // Level-triggered
    // Uncomment below for edge-triggered
    event.events = EPOLLIN | EPOLLET;

    event.data.fd = pipefd[0];

    if(epoll_ctl(epfd, EPOLL_CTL_ADD, pipefd[0], &event) == -1) {
        perror("epoll_ctl");
        exit(EXIT_FAILURE);
    }

    // 1. 生产者向管道写入 数据
    write(pipefd[1], "Hello epoll!", 13);

    // 2. 消费者在管道上调用 epoll_wait(), 等待 pipe 出现数据
    // 对于水平触发的监听: 步骤 2 对 epoll_wait() 的调用将立即返回 (一个状态发生)
    // 对于边缘触发的监听, 这个调用直到步骤 1 发生后才会返回 (状态改变的时候才会产生)
    for(;;) {
        int nfds, i;
        nfds = epoll_wait(epfd, events, 10, -1);
        if (nfds < 0) {
            perror("epoll_wait");
            exit(EXIT_FAILURE);
        }
        for (i = 0; i < nfds; ++i) {
            if (events[i].data.fd = pipefd[0]) {
                ssize_t n = read(pipefd[0], buf, BUF_LEN - 1);
                buf[n] = '\n';
                std::cout << buf << std::endl;
                // Delay to demonstrate LT vs ET behavior
                sleep(1);
            }
        }
    }

    close(epfd);
    close(pipefd[0]);
    close(pipefd[1]);

    return EXIT_SUCCESS;
}

int main()
{   
    // int epfd = labOne();
    // labTwo(epfd);
    // labThree(epfd);

    labFour();

    return 0;
}