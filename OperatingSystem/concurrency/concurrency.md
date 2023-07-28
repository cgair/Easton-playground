# Concurrency: An Introduction
A classic process(which we can now call a single-threaded process), there is a single stack, usually residing at the bottom of the address space.
However, in a multi-threaded process, each thread runs independently and of course may call into various routines to do whatever work it is doing. 

Instead of a single stack in the address space, there will be one per thread:
```text
+-------------------+ 0 KB
|   Program Code    |
|-------------------| 1 KB
|       Heap        |
|-------------------| 2 KB
|                   |
|       (free)      |
|                   |
+-------------------+
|      Stack(2)     |
|-------------------|
|      (free)       |
|-------------------| 15KB
|      Stack(1)     |
+-------------------+ 16KB
```
# 并发控制: 互斥
## [互斥.Q1: 如何在多处理器上实现线程互斥?]
实现互斥的根本困难: **不能同时读/写共享内存**

硬件能为我们提供一条 "瞬间完成" 的读 + 写指令

* [stdatomic.h](https://en.cppreference.com/w/cpp/header/stdatomic.h) (C11)
## 自旋锁 (Spin Lock)

**自旋锁的缺陷: (性能问题)**
* 自旋 (共享变量) 会触发处理器间的缓存同步，延迟增加
* 除了进入临界区的线程, 其他处理器上的线程都在空转
* 获得自旋锁的线程可能被操作系统切换出去 (实现 100% 的资源浪费)

## 互斥锁 (Mutex Lock)
如何实现线程 + 长临界区的互斥 ?
* 与其干等, 不如把自己 (CPU) 让给其他线程执行.

"让" 不是 C 语言代码可以做到的 (C 代码只能计算) --> 一个用户线程如果可以阻止自己被切走岂不是很恐怖
* 把锁的实现放到操作系统里就好啦!
* syscall(SYSCALL_lock, &lk);
  * 试图获得 lk，但如果失败，就切换到其他线程
* syscall(SYSCALL_unlock, &lk);
  * 释放 lk，如果有等待锁的线程就唤醒

## Futex: Fast Userspace muTexes
回顾性能优化的最常见技巧: 看 average (frequent) case 而不是 worst case.
* Spin Lock
  * Fast path: 一条原子指令, 上锁成功立即返回
  * Slow path: 上锁失败, 执行系统调用睡眠
```bash
# 观察
gcc sum-scalability-spin.c 
# gcc sum-scalability-mutex.c 
time ./a.out 16
```
# 并发控制: 同步
典型的同步问题: 生产者-消费者; 哲学家吃饭.
同步的实现方法: 信号量, 条件变量
## [同步.Q1 - 如何在多处理器上协同多个线程完成任务?]

## 线程同步 (Synchronization)
两个或两个以上随时间变化的量在变化过程中保持一定的相对关系

### 生产者-消费者问题
> 99% 的实际并发问题都可以用生产者-消费者解决.
生产者 --> 缓冲区 --> 消费者
* 生产者在生成数据后, 放在一个缓冲区中; 消费者从缓冲区取出数据处理; 任何时刻, 只能有一个生产者或消费者可以访问缓冲区.

### 条件变量: 万能同步方法
> 任何同步问题都有先来先等待的条件.
```c
void Tproduce() {
  while (1) {
retry:                    // ----
    mutex_lock(&lk);      //    ｜
    if (count == n) {     //    ｜- 希望把 spinning 改成释放锁并睡眠,
      mutex_unlock(&lk);  //    ｜  然后在某个时刻被唤醒
      goto retry;         // ----
    }
    count++;
    printf("(");
                          // wakeup()
    mutex_unlock(&lk);
  }
}
```

#### 条件变量 API
* wait(cv, mutex) 💤
    * 调用时必须保证已经获得 mutex
    * 释放 mutex、进入睡眠状态
* signal/notify(cv) 💬 私信: 走起
    * 如果有线程正在等待 cv, 则唤醒其中一个线程
* broadcast/notifyAll(cv) 📣 所有人: 走起
    * 唤醒全部正在等待 cv 的线程

#### 万能的同步方式
* 需要等待条件满足时
```c
mutex_lock(&mutex);
while (!cond) {
  wait(&cv, &mutex);
}
assert(cond);
// ...
// 互斥锁保证了在此期间条件 cond 总是成立
// ...
mutex_unlock(&mutex);
```
* 其他线程条件可能被满足时
```c
broadcast(&cv);
```

#### 条件变量: 实现并行计算
Job queue 可以实现几乎任何并行算法
```c
// 一个线程可以独立完成的很小的任务
struct job {
  void (*run)(void *arg);
  void *arg;
}

while (1) {
  struct job *job;

  mutex_lock(&mutex);
  while (! (job = get_job()) ) { // 没有 job 就 wait
    wait(&cv, &mutex);
  }
  mutex_unlock(&mutex);

  job->run(job->arg); // 不需要持有锁
                      // 可以生成新的 job
                      // 注意回收分配的资源
}
```

### 信号量
回想更衣室管理问题: (操作系统 = 更衣室管理员)

**互斥锁**
* 先到的人 (线程)
  * 成功获得手环, 进入游泳馆
  * *lk = 🔒, 系统调用直接返回
* 后到的人 (线程)
  * 不能进入游泳馆, 排队等待
  * 线程放入等待队列, 执行线程切换 (yield)
* 洗完澡出来的人 (线程)
  * 交还手环给管理员; 管理员把手环再交给排队的人
  * 如果等待队列不空, 从等待队列中取出一个线程允许执行
  * 如果等待队列为空, *lk = ✅

管理员 (OS) 使用自旋锁确保自己处理手环的过程是原子的

实际上, 完全没有必要限制手环的数量 (让更多同学可以进入更衣室)
* 管理员可以持有任意数量的手环 (更衣室容量上限)
  * 先进入更衣室的同学先得到
  * 手环用完后才需要等同学出来

**信号量 (semaphore)**拓展了互斥锁 (一把钥匙), 现在有多把钥匙.
* "手环" = "令牌" = "一个资源" = "信号量"

信号量设计的重点: 考虑 "手环" (每一单位的 "资源") 是什么, 谁创造? 谁获取?
* 在 "一单位资源" 明确的问题上更好用
#### P/V 操作
* P 操作: 将 sem 减 1, 相减后, 如果 sem < 0, 则进程/线程进入阻塞等待, 否则继续, 表明 P 操作可能会阻塞;
* V 操作: 将 sem 加 1, 相加后, 如果 sem <= 0, 唤醒一个等待中的进程/线程, 表明 V 操作不 会阻塞;

P 操作是用在进入临界区之前, V 操作是用在离开临界区之后, 这两个操作是必须成对出现的.

### 哲学家吃饭问题
哲学家 (线程) 有时思考, 有时吃饭
![](https://jyywiki.cn/pages/OS/img/dining-philosophers.jpg)
* 吃饭需要同时得到左手和右手的叉子
* 当叉子被其他人占有时, 必须等待, 如何完成同步?
  * 如何用互斥锁/信号量实现?

