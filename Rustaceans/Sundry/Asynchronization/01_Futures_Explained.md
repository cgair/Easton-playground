[toc]
本文摘抄自 [Futures Explained in 200 Lines of Rust](https://cfsamson.github.io/books-futures-explained/0_background_information.html)

# Some Background Information



## Green threads/stackful coroutines

Green threads use the same mechanism as an OS - creating a thread for each task, setting up a stack, saving the CPU's state, and jumping from one task(thread) to another by doing a "context switch".

我们将控制权交给调度程序(scheduler, which is a central part of the runtime in such a system) ，然后调度程序继续运行不同的任务。

**The typical flow looks like this:** 

1. 运行一些非阻塞代码 
2. 对某些外部资源进行阻塞调用 
3. CPU “跳转”到 “main”线程，该线程调度一个不同的线程来运行，并“跳转”到该栈中 
4. 在新线程上运行一些非阻塞代码，直到新的阻塞调用或任务完成 
5. CPU “跳转”回 “main” 线程 ，调度一个新线程 which is ready to make progress, 然后“跳转”到该线程

These "跳转" are known as **context switches**. Your OS is doing it many times each second.



## Callback based approaches

> // TODO









# Futures in Rust

**Async in Rust uses a `Poll` based approach, in which an asynchronous task will have three phases.**

1. 轮询阶段` (The Poll phase)`：一个 `Future` 被轮询，会开始执行，直到 it can no longer make progress.. 我们经常将运行时中轮询` Future `的部分称为执行器`(executor)` 
2. 等待阶段`(The Wait phase)`： 事件源 `(event source, 通常称为 reactor)` 注册一个` Future `正在等待一个事件发生，并确保它会在该事件准备好时唤醒相应的`Future` 
3. 唤醒阶段`(The Wake phase)`： 事件发生，相应的`Future`被唤醒。现在由在步骤 1 中轮询 `Future` 的 `executor `来安排再次轮询` Future`并 make further progress 直到它完成或到达一个阻塞点，如此往复，直到最终完成

## Leaf futures

```rust
// stream is a **leaf-future**
let mut stream = tokio::net::TcpStream::connect("127.0.0.1:3000");
```

Operations on these resources, like a `Read` on a socket, will be non-blocking 并返回一个我们称之为`leaf-future`的`Future`，这是我们实际上正在等待的`Future`.

It's **unlikely that you'll implement a leaf future yourself** unless you're writing a runtime.

It's **also unlikely that you'll pass a leaf-future to a runtime and run it to completion alone**.



## Non-leaf-futures

```rust
// Non-leaf-future
let non_leaf = async {
    let mut stream = TcpStream::connect("127.0.0.1:3000").await.unwrap();// <- yield
    println!("connected!");
    let result = stream.write(b"hello world\n").await; // <- yield
    println!("message sent!");
    ...
};
```

Non-leaf-futures are the kind of futures we as *users* of a runtime write 用`async`关键字创建的`Future` to create a **task** which can be run on the executor.

异步程序的大部分 will consist of non-leaf-futures, 这是一种可暂停的计算. 这是一个重要的区别，因为这些`Future`代表a *set of operations*。 这样的任务由 `await` 一系列 `leaf-future `组成。

The key to these tasks is that they're able to yield control to the runtime's scheduler and then resume execution again where it left off at a later point.

 与`leaf-future`相比，这些`Future`本身并不代表I/O资源。When we poll them they will run until they get to a leaf-future which returns `Pending` and then yield control to the scheduler (which is a part of what we call the runtime). 然后等待相关资源`ready`的时候唤醒自己

## I/O vs CPU intensive tasks

```rust
let non_leaf = async {
    let mut stream = TcpStream::connect("127.0.0.1:3000").await.unwrap(); // <-- yield

    // request a large dataset
    let result = stream.write(get_dataset_request).await.unwrap(); // <-- yield

    // wait for the dataset
    let mut response = vec![];
    stream.read(&mut response).await.unwrap(); // <-- yield

    // do some CPU-intensive analysis on the dataset
    let report = analyzer::analyze_data(response).unwrap();

    // send the results back
    stream.write(report).await.unwrap(); // <-- yield
};
```

两个`yield`之间的代码与我们的`executor`在同一个线程上运行。这意味着当我们分析器处理数据集时，执行器忙于计算而不是处理新的请求。

**something you must be aware of:**

1. We could create a new leaf future which 它将我们的任务发送到另一个线程并 resolves when the task is finished. We could `await` this leaf-future like any other future.
2. The runtime could have some kind of supervisor that monitors how much time different tasks take, 并将执行器本身移动到不同的线程 so it can continue to run even though our `analyzer` task is blocking the original executor thread.
3. You can 创建一个与运行时兼容的`reactor`,  which does the analysis any way you see fit, and returns a Future which can be awaited.



[A high level mental model of how Futures and runtimes work ](https://cfsamson.github.io/books-futures-explained/2_a_mental_model_for_futures.html)



# Generators and async/await

- Understand how the async/await syntax works under the hood



#  Pin

Pin wraps a pointer. A reference to an object is a pointer.





# Postscripts

## coroutines

协程`(coroutines)`[wiki](https://en.wikipedia.org/wiki/Coroutine)

协程是自己程序调度的，逻辑上并行执行，底层上非并行执行
线程是操作系统调度的，逻辑和底层上都是并行执行

协程的切换开销远小于内核对于线程的切换开销

> 因此对于 IO 密集型的程序，使用协程的优势在于用户可以像写同步 IO 一样，无需关心 IO 异步接口的细节（coroutine 库进行调度）。减小使用多个线程做同步 IO 带来的内核大量线程切换的开销

协程最重要的几个特征

> 用户控制切换，自动保存上下文状态，切换之间可以通过参数通信，可用同步的方式实现异步。
