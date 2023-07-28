# Green Threads
[Green Threads](https://cfsamson.gitbook.io/green-threads-explained-in-200-lines-of-rust/green-threads)解决了编程中的一个常见问题: 您不希望您的代码阻塞 CPU, 从而阻止 CPU 进行有意义的工作.
不要与并行性混淆. Think of it this way, Green Threads 让我们更聪明、更高效地工作, 从而更有效地使用我们的资源, 并行性就像在问题上投入更多资源.
我们通过使用 multitasking(多任务处理) 来解决这个问题, 这让我们 suspend 一段代码的执行, 同时恢复另一段代码并在 "contexts" 之间切换. 有两种方法可以做到这一点: 
* Preemptive multitasking
* Non-preemptive multitasking (or cooperative multitasking)

## Preemptive multitasking
// TODO

## Non-preemptive multitasking
一个任务自己决定什么时候 CPU 做其他事情比等待当前任务中发生的事情更好. 一个例子是在发生会阻止执行的事情 (IO operation) 时交出 cpu 控制权.
