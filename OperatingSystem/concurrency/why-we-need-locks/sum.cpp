#include <thread>
#include <iostream>

#define N 10000 // 对共享数据自增的次数
int sum = 0;  // 共享数据

extern "C" void tSum() {
    // for (int i = 0; i < N; ++i) {
    //     sum ++;
    // }
    int i = 0;      /* PC = 1 */
    while (i < N) { /* PC = 2 */
        sum ++;     /* PC = 3 */
        i++;        /* PC = 4 */
    }
}

// 1. g++ sum.cpp: 13945, 17159, 20000 每次结果都不一样
// 2. 添加编译优化 -o1 -o2: 实际上 g++ 并没有优化 tSum()
int main()
{
    std::thread t1(tSum);
    std::thread t2(tSum);

    t1.join();
    t2.join();

    std::cout << "Now i = "
              << sum << std::endl;

    return 0;
}