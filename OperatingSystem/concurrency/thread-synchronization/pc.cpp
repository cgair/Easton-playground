#include <thread>
#include <mutex>
#include <condition_variable>
#include <iostream>
#include <vector>

#define N 10

std::mutex mtx;
std::condition_variable cv;
int buffer[N];
int head = 0, rear = 0;

void Tproducer(int prod) {
    while (1) {
       std::unique_lock<std::mutex> lck(mtx); 
       while ((rear + 1) % N == head) {
        cv.wait(lck);
       }
       buffer[rear] = prod;
       rear = (rear + 1) % N;
       cv.notify_all();
       lck.unlock();
    }
}

void Tconsumer() {
    while (1) {
        std::unique_lock<std::mutex> lck(mtx);
        while (head == rear) {
            cv.wait(lck);
        }
        std::cout << buffer[head] << std::endl;
        head = (head + 1) % N;
        cv.notify_all();
        lck.unlock();
    }
}


int main()
{
    std::vector<std::thread> thread_pool;
    for (int i = 0; i < 8; ++i) {
        thread_pool.push_back(std::thread(Tproducer, i));
        thread_pool.push_back(std::thread(Tconsumer));
    }

    for (auto &t : thread_pool) {
        t.join();
    }

    return 0;
}

// See also:
// <https://www.zywvvd.com/notes/coding/cpp/cpp-producer-consumer/cpp-producer-consumer/>