**VIRTUALIZATION**
操作系统为 (所有) 程序提供 API
* 进程 (状态机) 管理
  * fork, execve, exit - 状态机的创建/改变/删除
* 存储 (地址空间) 管理
  * mmap - 虚拟地址空间管理
* 文件 (数据对象) 管理
  * open, close, read, write - 文件访问管理
  * mkdir, link, unlink - 目录管理
# 操作系统上的进程 (Processes)
## [操作系统上的进程.Q1 - 操作系统启动后到底做了什么?]
## [操作系统上的进程.Q2 - 操作系统如何管理程序 (进程)?]
## [操作系统上的进程.Q3 - Zombie and Orphan Processes?]
当进程 exit() 退出之后, 他的父进程没有通过 wait() 系统调用回收其进程描述符的信息, 该进程会继续停留在系统的进程表中, 占用内核资源, 这样的进程就是僵尸进程.
当一个进程正在运行时, 他的父进程忽然退出, 此时该进程就是一个孤儿进程.

孤儿进程会由 init 进程收养作为子进程, 所以不会有什么危害;
僵尸进程会占用进程号, 以及未回收的文件描述符占用空间, 如果产生大量的僵尸进程, 会导致系统无法分配进程号, 说明父进程的代码编写有问题.
```bash
ps -aux | grep Z
```

操作系统启动后到底做了什么?

CPU Reset → Firmware → Boot loader → Kernel _start()

实际上真正的操作系统就是**加载"第一个程序"** (`pstree` 操作系统创建树根的那个进程)
* [RTFSC](https://elixir.bootlin.com/linux/latest/source/init/main.c#L1582) (latest Linux Kernel 是怎么做的)
  * 如果没有指定启动选项 init=, 按照"默认列表"尝试一遍
  * 在我的机器上, `ll /sbin/init` 看到 `/sbin/init -> /lib/systemd/systemd* (pstree 树根)`
  * 至此, Linux Kernel 就进入后台, 成为 "中断/异常处理程序"
所以完整流程即: CPU Reset → Firmware → Loader → Kernel _start() 
→ 第一个程序 /sbin/init → 程序 (状态机) 执行 + 系统调用

## fork()
做一份状态机完整的复制 (内存、寄存器现场)
* 新创建进程返回 0, 执行 fork 的进程返回子进程的进程号
* 因此总能找到 "父子关系", 因此有了进程树 (`pstree`)

[理解fork - fork-demo.c (画出状态机的转移)]
[理解fork - fork-printf.c]
* 一切状态都会被复制

sh-xv6.c
* fork + execve + pipe: UNIX Shell 的经典设计
* fork 状态机复制包括持有的所有操作系统对象
* execve "重置" 状态机, 但继承持有的所有操作系统对象 (文件描述符并没有 reset)

### 复制, 但又没完全复制
概念上状态机被复制, 但实际上复制后内存都被共享.
* "Copy-on-write" 只有被写入的页面才会复制一份
  * 被复制后, 整个地址空间都被标记为 "只读"
  * 操作系统捕获 Page Fault 后酌情复制页面
  * 好处就是: 比如多个应用程序在不同的地址空间映射 libc, 而整个系统里只有一份 libc 的物理内存的覆盖.
  * cow-test.c: 128MB 代码 + 128MB 数据, 创建 1000 个进程. (所以**统计进程占用内存是个伪命题**)
P.S. 一个合理的定义进程内存占用的方法是映射了多少虚拟内存 (`pmap`)

### fork 可以创造平行宇宙
* 创造平行宇宙: 搜索并行化, 加速状态空间搜索
* 创造平行宇宙: 跳过初始化
* 创造平行宇宙: 备份和容错 -> 用 fork() 做快照, 主进程 crash 了, 启动快照重新执行.

## execve()
光有 `fork` 还不够, 怎么"执行别的程序"?
```c
#include <unistd.h>
int execve(const char *pathname, char *const argv[],
          char *const envp[]);
```

## _exit()
有了 fork, execve 我们就能自由执行任何程序了, 最后只缺一个销毁状态机的函数!
* exit syscall 世界里所有的东西都保持不变, 除了执行 exit 的状态机从系统中消失.

**exit 的几种写法 (它们是不同)**
* exit(0) - stdlib.h 中声明的 libc 函数
  * 会调用 atexit
* _exit(0) - glibc 的 syscall wrapper
  * 执行 "exit_group" 系统调用终止整个进程 (所有线程)
  * 不会调用 atexit
* syscall(SYS_exit, 0)
  * 执行 "exit" 系统调用终止当前线程
  * 不会调用 atexit
P.S. atexit()用来注册一些在进程结束时要调的函数

### 进程终结
进程通过 exit() 系统调用 (可能是来自进程内部的exit(), 也可能来自外部的信号) 结束进程,释放他所占有的所有资源 (包括引用的文件, 内存描述符, 还会给自己的父进程发送信号, 给自己的子进程寻找一个父进程等操作).
调用结束后该进程并没有完全从系统上消失, 进程的进程描述符依然存在于系统中, 存在的唯一目的就是向父进程提供信息.
进程的收尾工作总是由该进程的父进程来做的, 父进程通过 wait() 系统调用来释放该进程最后剩余的进程标识符, slab缓存等, 该调用会阻塞当前父进程, 直到某个子进程退出.
#### [Lab - 进程退出]
```bash
# 1. ps 获取 bash 进程的 PID
#   PID TTY          TIME CMD
# 55744 pts/47   00:00:00 bash

# 2. 新开一个 bash, 查看上个 bash 进程的系统调用
sudo strace -p <Pid>

# 3. 输入一条命令回车, 观察系统调用情况
# 以 tail 为例, 可以看到阻塞到该系统调用, 也就是在等待回收子进程.
# rt_sigprocmask(SIG_SETMASK, [], NULL, 8) = 0
# rt_sigprocmask(SIG_BLOCK, [CHLD], [], 8) = 0
# wait4(-1, 

# 4. Ctrl-C 结束进程 tail 时, 返回了 tail 进程的 Pid.
# wait4(-1, [{WIFSIGNALED(s) && WTERMSIG(s) == SIGINT}], WSTOPPED|WCONTINUED, NULL) = 79308
```


# 进程的地址空间 (Address Spaces)
Allowing multiple programs to reside concurrently in memory makes protection an important issue; you don’t want a process to be able to read, or worse, write some other process’s memory.
|
|-Requires the OS to create an easy to use abstraction of physical memory. (The Address Space)
## [进程的地址空间.Q1 - 进程的地址空间是如何创建、如何更改的?]
## [进程的地址空间.Q2 - 进程地址空间的管理 API?] mmap
pmap (1) - report memory of a process
```bash
# 运行 minimal.S
pmap <pid>
# 0000000000400000      4K r---- a.out
# 0000000000401000      4K r-x-- a.out  # 可读可执行, 第一条指令在 401000 (查看 assambly 发现确实是一条 mov 指令)
# 00007ffff7ff9000     16K r----   [ anon ]
# 00007ffff7ffd000      8K r-x--   [ anon ]
# 00007ffffffde000    132K rw---   [ stack ]
# ffffffffff600000      4K --x--   [ anon ]
#  total              168K
```
### [进程的地址空间.Lab1 - 进程地址空间]
动态链接的 binary
```c
// 只定义一个 main 函数
// 静态和动态链接查看 /proc/<pid>/maps
int main()
{
    return 0;
}
// 动态链接
// 555555554000-555555555000 r--p 00000000 08:02 96869955                   a.out
// 555555555000-555555556000 r-xp 00001000 08:02 96869955                   a.out
// 555555556000-555555557000 r--p 00002000 08:02 96869955                   a.out
// 555555557000-555555558000 r--p 00002000 08:02 96869955                   a.out
// 555555558000-555555559000 rw-p 00003000 08:02 96869955                   a.out
// 7ffff7d88000-7ffff7d8b000 rw-p 00000000 00:00 0                          (这是什么?)
// 7ffff7d8b000-7ffff7db3000 r--p 00000000 08:02 77070959                   libc.so.6
// 7ffff7db3000-7ffff7f48000 r-xp 00028000 08:02 77070959                   libc.so.6
// 7ffff7f48000-7ffff7fa0000 r--p 001bd000 08:02 77070959                   libc.so.6
// 7ffff7fa0000-7ffff7fa4000 r--p 00214000 08:02 77070959                   libc.so.6
// 7ffff7fa4000-7ffff7fa6000 rw-p 00218000 08:02 77070959                   libc.so.6
// 7ffff7fa6000-7ffff7fb3000 rw-p 00000000 00:00 0                          
// 7ffff7fbb000-7ffff7fbd000 rw-p 00000000 00:00 0 
// 7ffff7fbd000-7ffff7fc1000 r--p 00000000 00:00 0                          [vvar] (这又是什么?)
// 7ffff7fc1000-7ffff7fc3000 r-xp 00000000 00:00 0                          [vdso] (这叒是什么?)
// 7ffff7fc3000-7ffff7fc5000 r--p 00000000 08:02 77070351                   ld-linux-x86-64.so.2
// 7ffff7fc5000-7ffff7fef000 r-xp 00002000 08:02 77070351                   ld-linux-x86-64.so.2
// 7ffff7fef000-7ffff7ffa000 r--p 0002c000 08:02 77070351                   ld-linux-x86-64.so.2
// 7ffff7ffb000-7ffff7ffd000 r--p 00037000 08:02 77070351                   ld-linux-x86-64.so.2
// 7ffff7ffd000-7ffff7fff000 rw-p 00039000 08:02 77070351                   ld-linux-x86-64.so.2
// 7ffffffde000-7ffffffff000 rw-p 00000000 00:00 0                          [stack]
// ffffffffff600000-ffffffffff601000 --xp 00000000 00:00 0                  [vsyscall] (这叕是什么?)
```
可能 `pmap` 结果看起来更友好:
```text
0000555555554000      4K r---- a.out
0000555555555000      4K r-x-- a.out
0000555555556000      4K r---- a.out
0000555555557000      4K r---- a.out
0000555555558000      4K rw--- a.out
00007ffff7d88000     12K rw---   [ anon ] (这是什么?)
00007ffff7d8b000    160K r---- libc.so.6
00007ffff7db3000   1620K r-x-- libc.so.6
00007ffff7f48000    352K r---- libc.so.6
00007ffff7fa0000     16K r---- libc.so.6
00007ffff7fa4000      8K rw--- libc.so.6
00007ffff7fa6000     52K rw---   [ anon ]
00007ffff7fbb000      8K rw---   [ anon ]
00007ffff7fbd000     16K r----   [ anon ]
00007ffff7fc1000      8K r-x--   [ anon ]
00007ffff7fc3000      8K r---- ld-linux-x86-64.so.2
00007ffff7fc5000    168K r-x-- ld-linux-x86-64.so.2
00007ffff7fef000     44K r---- ld-linux-x86-64.so.2
00007ffff7ffb000      8K r---- ld-linux-x86-64.so.2
00007ffff7ffd000      8K rw--- ld-linux-x86-64.so.2
00007ffffffde000    132K rw---   [ stack ]
ffffffffff600000      4K --x--   [ anon ]
 total             2644K
```
(这是什么?) --> 是不是 bss? 加一个大数组试试:
```c
int big[1 << 30]; // 1GB
int main()
{
    return 0;
}
// 0000555555554000      4K r---- a.out
// 0000555555555000      4K r-x-- a.out
// 0000555555556000      4K r---- a.out
// 0000555555557000      4K r---- a.out
// 0000555555558000      4K rw--- a.out
// 0000555555559000 4194304K rw---   [ anon ] 确实是
```
(这叒是什么?) --> 
vdso (7) (Virtual system calls): 只读的系统调用也许可以不陷入内核执行.
无需陷入内核的系统调用

例子: time (2)
直接调试 vdso.c
时间: 内核维护秒级的时间 (所有进程映射同一个页面)

## 进程的地址空间管理
进程只有少量内存映射
* 静态链接：代码、数据、堆栈、堆区
* 动态链接：代码、数据、堆栈、堆区、INTERP (ld.so)
  * gdb 调试一个动态链接的 binary, `starti` 发现没有 libc.so, continue 发现 libc.so 出现了
地址空间里剩下的部分是怎么创建的? 创建了以后还能修改它吗?
* [mmap](https://man7.org/linux/man-pages/man2/mmap.2.html)


# 系统调用 和 UNIX Shell
## [UNIX Shell.Q1 - 那"我们"到底怎么用操作系统?]
我们是操作系统的用户; 但操作系统提供的 API 并不是 "我们" 作为人类用户能直接使用的.
## Shell
Shell 两大作用: 用户接口、Job control.
### Shell 提供用户接口
我们需要一个 "用户能直接操作" 的程序管理操作系统对象 -> 这就是 Shell (内核 Kernel 提供系统调用; Shell 提供用户接口)
* Shell 是一门 "把用户指令翻译成系统调用" 的编程语言 (`man sh`)。
* sh-xv6.c
  * 零库函数依赖 (-ffreestanding 编译、ld 链接) 「-ffreestanding 用于指示编译器生成一个不依赖于操作系统提供的标准库的可执行文件或静态库」
### 终端和 Job Control
Shell 未解之谜:

1. 为什么 `Ctrl-C` 可以退出程序?
2. 为什么有些程序又不能退出?
* 没有人 `read` 这个按键, 为什么进程能退出?
* `Ctrl-C` 到底是杀掉一个, 还是杀掉全部?
  * 如果我 `fork` 了一份计算任务呢?
  * 如果我 `fork-execve` 了一个 shell 呢?
3. 为什么 Tmux 可以管理多个窗口? Tmux 是如何实现的?

答案: **终端**

### Session, Process Group 和信号
![](https://jyywiki.cn/pages/OS/img/process-groups-sessions.png)
Controlling terminal
* 用户登录计算机, 登陆进程会为用户创建一个session (只有用户的登录 shell 进程, 作为 seesion leader), session 分配给用户一个 controlling terminal, session 里有很多个进程组(process group), 这些前台的进程组里的进程无论 fork 多少个进程, 都属于一个进程组 (继承 PGID), 所以 `Ctrl-C` 发给进程组内所有进程.

From [man setpgid](https://man7.org/linux/man-pages/man2/setpgid.2.html)
> A session can have a controlling terminal. 
> At any time, one (and only one) of the process groups in the session can be the foreground process group for the terminal; the remaining process groups are in the background.

信号
* SIGINT/SIGQUIT
* 大家熟悉的 Segmentation Fault/Floating point exception (core dumped)
  * #GP, #PG, #DIV (UNIX 系统会给进程发送一个信号, 此时可以生成一个"core"文件「ELF格式」, 能用gdb调试)

# C 标准库的实现
## [C 标准库的实现.Q1 - 如何在系统调用之上构建程序能够普遍受惠的标准库?]
**熟悉又陌生的 libc**
* [Reference](https://cplusplus.com/reference/)
## 封装(1): 纯粹的计算
RTFM:
* [stdlib.h](https://cplusplus.com/reference/cstdlib/)
* [math.h](https://www.cplusplus.com/reference/cmath/)
* 标准库只对"标准库内部数据"的线程安全性负责. (**C 标准库是线程安全的**)

## 封装(2): 文件描述符
libc 的第二个任务就是封装操作系统中的对象. 
* In UNIX Everything is a File. 
* 文件描述符: 一个指向操作系统内对象的 "指针"
```c
/* 得到一个文件描述符 */
int open(const char *pathname, int flags);
/* O_APPEND, ..., O_CLOEXEC */
```
* 对象只能通过操作系统允许的方式访问
* 从 0 开始编号 (0, 1, 2 分别是 stdin, stdout, stderr)
* 可以通过 open 取得; close 释放; dup "复制"
* 对于数据文件, 文件描述符会 "记住" 上次访问文件的位置
  * write(3, "a", 1); write(3, "b", 1);
* FILE * 背后其实是一个文件描述符 (package2.c).

## 封装(3): 更多的进程/操作系统功能 
env.c

## 封装(4): 地址空间
**libc 提供的很重要的机制**
在大区间 `[L, R)` 中维护互不相交的区间的集合
```text
内存区域                [li, ri]
 +---------------------------------------------+
 |///｜                 |///|                  | 
 +---------------------------------------------+
 [L                                            R)
```
`malloc` 和 `free` (区间管理的问题)
* malloc(s) - 返回一段大小为 s 的区间
  * 必要时可以向操作系统申请额外的 `[L, R)` (观察 strace)
  * 允许在内存不足时 "拒绝" 请求
* `free(l, r)` - 给定 `l`, 删除 `[l, r)`
实现高效的 malloc/free
>Premature optimization is the root of all evil. ——D. E. Knuth
脱离 workload 做优化就是耍流氓.
[Mimalloc: free list sharding in action (APLAS'19)](https://www.microsoft.com/en-us/research/uploads/prod/2019/06/mimalloc-tr-v1.pdf)

**指导思想**
* 越小的对象创建/分配越频繁
* 较为频繁地分配中等大小的对象
* 低频率的大对象
* 所有分配都会在所有处理器上发生 (并行)
指导思想: O(n) 大小的对象分配后至少有 Ω(n) 的读写操作, 否则就是 performance bug (不应该分配那么多).

**现实世界中的 malloc/free**
当然, 实际情况会复杂一些, 性能也是锱铢必较
* glibc: arena → heap → tcache (thread-local)
* tcmalloc: thread-caching malloc, mimalloc

## 无止境地封装 (更多的功能)
[The GNU C Library](https://www.gnu.org/software/libc/manual/html_mono/libc.html)
```text
+----------------------------------------+
|   计算机世界 openjdk CPython C++ Rust    |
|              ...      |                ｜
|  +--------------------V-------+        ｜
|  |                 libc + ABI |        ｜ 
|  | +-------------+            |        ｜
|  | |    Kernel   | syscall    |        ｜
|  | +-------------+            |        ｜
|  +----------------------------+        ｜
+----------------------------------------+
从系统调用 -> libc -> shell -> 应用的"软件栈"
```

# 可执行文件
## [可执行文件.Q1 - 可执行文件到底是什么?]
可执行文件: 一个描述了状态机的数据结构
## [可执行文件.Q2 - 可执行文件是如何在操作系统上被执行的?]
[System V ABI: System V Application Binary Interface (AMD64 Architecture Processor Supplement)](https://jyywiki.cn/pages/OS/manuals/sysv-abi.pdf)

可执行文件: 状态机的描述
一个描述了状态机的初始状态 + 迁移的**数据结构**
数据结构各个部分定义: `/usr/include/elf.h`

* 寄存器
  * 大部分由 ABI 规定, 操作系统负责设置
  * 例如初始的 PC
* 地址空间
  * 二进制文件 + ABI 共同决定
  * 例如 argv 和 envp (和其他信息) 的存储
* 其他有用的信息 (例如便于调试和 core dump 的信息)

常见的可执行文件 (UNIX/Linux):
* ELF (Executable Linkable Format)
* She-bang 
  * She-bang 其实是一个 “偷换参数” 的 execve (加载器发现 `#!` 时「比如`#!/usr/bin/python`」, 会把文件`/usr/bin/python`传给 execve 的第一个参数, 后边的传给第二个参数)

## [Q - 是谁决定了一个文件能不能执行?]
```bash
chmod -x a.out && ./a.out
# fish: The file “./a.out” is not executable by this user
# bash: ./a.out: Permission denied

chmod +x a.c && ./a.c
# Failed to execute process './a.c'. Reason:
# exec: Exec format error
# The file './a.c' is marked as an executable but could not be run by the operating system.
```
操作系统代码 **(execve)** 决定的
### [Lab - strace 看到失败的 execve]
1. `chmod -x ELF/a.out && strace ELF/a.out`
   * 没有执行权限的 a.out: execve = -1, EACCESS
2. `chmod +x ELF/a.c && strace ELF/a.c`
   * 有执行权限的 a.c: execve = -1, ENOEXEC (Exec format error)
3. 读 execve (2) 的手册

## 解析可执行文件
Binutils - Binary Utilities: [GNU binutils](https://www.gnu.org/software/binutils/)

分析可执行文件
* objcopy/objdump/readelf (计算机系统基础)
* addr2line, size, nm

### [Q - 为什么 gdb 知道出错的位置?]
```bash
gdb segfault.out
(gdb) bt
```
因为应用程序二进制文件里包含额外的信息帮助 debugger 解析运行时状态.
将一个 assembly (机器) 状态映射到 "C 世界" 状态的函数
[The DWARF Debugging Standard](https://dwarfstd.org/)

### [Lab - Stack Unwinding]
ELF/unwind.c
需要的编译选项
* -g (生成调试信息)
* -static (静态链接)
* -fno-omit-frame-pointer (总是生成 frame pointer)
(可以尝试不同的 optimization level 再 gdb)
**under the hood (x86):**
```bash
  401ce4:       55                      push   %rbp
  401ce5:       48 89 e5                mov    %rsp,%rbp
  401ce8:       b8 00 00 00 00          mov    $0x0,%eax
  401ced:       e8 d9 ff ff ff          callq  401ccb <foo>
```
函数调用时, call <...>, call 在栈上留下一个 return address
```text
call    |         |
|------>+---------+
        | retaddr |
        +---------+ <---- rsp 1
        | old rbp | (push %rbp)         
        +---------+ <---- rsp 2 <---- rbp (mov %rsp,%rbp) <-----------+
        |  local  |                                                   |
        |   vars  |                                                   | 
        +---------+ <---- rsp 3 (rsp 继续移动) **继续函数调用时**         |
        | retaddr |                                                   |
        +---------+ <---- rsp 4                                       | 
        | old rbp | --------------------------------------------------+
        +---------+ <---- rsp 5 <---- rbp 
```
## 编译和链接 (从 C 代码到二进制文件)
编译器 (gcc)
* High-level semantics (C 状态机) → low-level semantics (汇编)
汇编器 (as)
* Low-level semantics → Binary semantics (状态机容器)
  * "一一对应" 地翻译成二进制代码 (sections, symbols, debug info)
  * 不能决定的要留下 "之后怎么办" 的信息 (relocations)
链接器 (ld)
* 合并所有容器, 得到 "一个完整的状态机"
  * ldscript (-Wl,--verbose **「给链接器传 verbose 参数查看细节」**); 和 C Runtime Objects (CRT) 链接
  * missing/duplicate symbol 会出错

## 动态链接和加载
### [Q - 什么是动态链接/动态加载?]
```text
+----+----+----+----+
| .  | .  |////|////| a.out (ELF 文件)
+-|--+-|--+----+----+
  |    V     |
  V   PHT    | loader
ELF Header   |
             V
            execve("a.out")
```
### [Lab - 实现 ELF Loader]
ELF/loader-static.c (运行在用户态, 应用 mmap): 解析数据结构 + 复制到内存 + 跳转

# 上下文切换
## [上下文切换.Q1 - 操作系统是如何完成进程之间的切换的?]
机制 (mechanism): 上下文切换
* 在中断/系统调用时执行操作系统代码
* 操作系统实现所有状态机 (进程) 一视同仁的 "封存"
* 从而可以恢复任意一个状态机 (进程) 执行

## 处理器的虚拟化
为什么死循环不能使计算机被彻底卡死?
* 硬件会发生中断
* 切换到操作系统代码执行
* 操作系统代码可以切换到另一个进程执行

## 状态的封存: 体系结构相关的处理
x86-64
* 中断/异常会伴随堆栈切换
  * 通过 TSS 指定一个 "内核栈"
  * 中断前的寄存器[保存在堆栈上](https://www.felixcloutier.com/x86/intn:into:int3:int1) (典型的 CISC 行为)

因为被封存, 我们的处理器可以选择把任何一个状态机恢复
* 机制: 允许在中断/异常返回时把任何进程加载回 CPU
* 策略: 处理器调度

# 处理器调度
## [处理器调度.Q1 - 中断后策略 (policy) 我们到底选哪个进程执行呢?]

## 简化的处理器调度问题
中断机制
* 处理器以固定的频率被中断
  * Linux Kernel 可以配置: make menuconfig -> Processor type and features -> Time frequency -> 100/250/300/1000Hz
  * 中断/系统调用返回时可以自由选择进程/线程执行

**处理器调度问题的简化假设**
* 系统中有一个处理器 (1970s) `lscpu`
* 系统中有多个进程/线程共享 CPU
* 包括系统调用 (进程/线程的一部分代码在 syscall 中执行)
* 偶尔会等待 I/O 返回, 不使用 CPU (通常时间较长)

#### [策略: Round-Robin]
![Round-Robin](https://jyywiki.cn/pages/OS/img/sched-rr.png)
假设当前 Ti 运行
* 中断后试图切换到下一个线程 T (i + 1) mod n
* 如果下一个线程正在等待 I/O 返回, 继续尝试下一个
  * 如果系统所有的线程都不需要 CPU, 就调度 idle 线程执行
中断之间的**线程执行**称为 "时间片" (time-slicing)

#### [策略: 引入优先级]
UNIX niceness: 1. `top 看 NI` 2. `man nice`
* -20 .. 19 的整数, 越 nice 越让别人得到 CPU
  * -20: 极坏; most favorable to the process
  *  19: 极好; least favorable to the process

* (所以就有了) 基于优先级的调度策略
RTOS: 坏人躺下好人才能上
Linux: nice 相差 10, CPU 资源获得率相差 10 倍 (大约)
**Try nice/renice**
```bash
taskset -c 0 nice -n 19 yes > /dev/null &
taskset -c 0 nice -n  9 yes > /dev/null &
# 然后 top -d 0.5 看 yes 表现是否符合上面说的: nice 相差 10, CPU 资源获得率相差 10 倍 (大约)
```

## 真实的处理器调度
实际上更多的情况类似于:
系统里有两个进程
* 交互式的 Vim, 单线程
* 纯粹计算的 (mandelbrot.c), 32 个线程

Round-Robin 就会出现问题: 
* Vim 花 0.1ms 处理完输入就又等输入了 (主动让出 CPU)
* Mandelbrot 使 Vim 在有输入可以处理的时候被延迟 (必须等当前的 Mandelbrot 转完一圈), 数百 ms 的延迟就会使人感到明显卡顿

### [策略: 动态优先级]
![Round-Robin 队列](https://jyywiki.cn/pages/OS/img/MLFQ.png)
* 设置若干个 Round-Robin 队列
  * 每个队列对应一个优先级
* 动态优先级调整策略
  * 优先调度高优先级队列
  * 用完时间片 -> 坏人
  * 让出 CPU I/O -> 好人

### [策略: Complete Fair Scheduling (CFS)]
> The Completely Fair Scheduler (CFS) is a process scheduler that was merged into the 2.6.23 (October 2007) release of the Linux kernel and is the default scheduler of the tasks of the SCHED_NORMAL class (i.e., tasks that have no real-time execution constraints).

试图去模拟一个 "Ideal Multi-Tasking CPU":
* "Ideal multi-tasking CPU" is a (non-existent :-)) CPU that has 100% physical power and which can run each task at precise equal speed, in parallel, each at 1/n. For example: if there are 2 tasks running, then it runs each at 50% physical power — i.e., actually in parallel.

* "让系统里的所有进程尽可能公平地共享处理器"
  * 为每个进程记录精确的运行时间
  * 中断/异常发生后, 切换到运行时间最少的进程执行
  * 下次中断/异常后, 当前进程的可能就不是最小的了

操作系统具有对物理时钟的 "绝对控制"
* 每人执行 1ms, 但好人的钟快一些, 坏人的钟慢一些
  * [vruntime (virtual runtime)](https://stackoverflow.com/questions/19181834/what-is-the-concept-of-vruntime-in-cfs)
  * vrt[i] / vrt[j] 的增加比例 = wt[j] / wt[i]
```c
const int sched_prio_to_weight[40] = {
  /* -20 */ 88761, 71755, 56483, 46273, 36291,
  /* -15 */ 29154, 23254, 18705, 14949, 11916,
  /* -10 */  9548,  7620,  6100,  4904,  3906,
  /*  -5 */  3121,  2501,  1991,  1586,  1277,
  /*   0 */  1024,   820,   655,   526,   423,
  /*   5 */   335,   272,   215,   172,   137,
  /*  10 */   110,    87,    70,    56,    45,
  /*  15 */    36,    29,    23,    18,    15,
};
```
#### CFS 的复杂性 (1): 新进程/线程
假设 P1, P2, P3 已经执行几个时间片, 此时 P1 fork -> P4, P4 分配多少时间?
子进程继承父进程的 vruntime
* 并且从 2.6.32 开始, [parent run first](https://lkml.org/lkml/2009/9/11/411)
```c
static void task_fork_fair(struct task_struct *p) {
  struct sched_entity *se = &p->se, *curr;
  ...
  rq_lock(rq, &rf);
  update_rq_clock(rq);
  cfs_rq = task_cfs_rq(current);
  curr = cfs_rq->curr;
  if (curr) {
    update_curr(cfs_rq);
    se->vruntime = curr->vruntime; // 继承父进程的 vruntime
  }
  place_entity(cfs_rq, se, 1);
  ...
}
```
#### CFS 的复杂性 (2): I/O
I/O (例如 1 分钟) 以后回来 vruntime 严重落后
* 为了赶上, CPU 会全部归它所有
Linux 的实现
* 被唤醒的进程获得 "最小" 的 vruntime (可以立即被执行)
* 曾经会给唤醒的进程一些额外的 vruntime, **现在没有了**
```c
if (renorm && curr)
  se->vruntime += cfs_rq->min_vruntime;
```

#### CFS 的复杂性 (3): 整数溢出
vruntime 有优先级的 "倍数"
* 如果溢出了 64-bit 整数怎么办？
  * a < b 不再代表 "小于"!
  * 
假设: 系统中最近、最远的时刻差不超过数轴的一半
* 我们可以比较它们的相对大小

```c
bool less(u64 a, u64 b) {
  return (i64)(a - b) < 0;
}
```

#### 实现 CFS 的数据结构
用什么数据结构维护所有进程的 vruntime? (延伸到: 根据需要选择合适的数据结构)
* 任何有序集合 (例如 binary search tree, linux kernel 用了红黑树) 维护线程 t 的 vrt(t)
  * 更新 vrt(t) <- vrt(t) + Δt/w 
  * 取最小的 vrt
  * 进程创建/退出/睡眠/唤醒时插入/删除 t

道理还挺简单的
  * 代码实现有困难
  * 还不能有并发 bug (又不能上大锁)

**是否解决了问题?**
我们之前的假设都是基于进程间不会协作.
考虑情况: Producer, Consumer, while (1)
```text
+--+---+-----------+---+---+-----------+
| P | C |           | P | C |           |
|   |   |  while(1) |   |   |  while(1) |
|   |   |           |   |   |           |
+---+---+-----------+---+---+-----------+----------------->
产生不公平: P, C 几乎不用时间片, while(1) 会把时间片用满.
```
### 真实的处理器调度
#### 优先级反转
```c
void xiao_zhang() { // 高优先级
  sleep(1); // 休息一下先
  mutex_lock(&wc);
  ...
}

void xi_zhu_ren() { // 中优先级
  while (1) ;
}

void jyy() { // 最低优先级
  mutex_lock(&wc);
  ...
}
// jyy 在持有互斥锁的时候被赶下了处理器...
// xi_zhu_ren 抢占了 cpu, 1 ms 后 xiao_zhang 抢占 cpu, 却在等锁, 发生了优先级的反转 (xiao_zhang 等 jyy, jyy 等 xi_zhu_ren -> xiao_zhang 等 xi_zhu_ren)
```
上面的情况真的出现过: [The First Bug on Mars](https://kwahome.medium.com/the-first-bug-on-mars-os-scheduling-priority-inversion-and-the-mars-pathfinder-53586a631525)
![The First Bug on Mars](https://jyywiki.cn/pages/OS/img/marsbot.png)
##### 解决优先级反转问题
Linux: CFS 凑合用吧
* 实时系统: 火星车在 CPU Reset 啊喂??
  * 优先级继承 (Priority Inheritance)/优先级提升 (Priority Ceiling)
    * 持有 mutex 的线程/进程会继承 block 在该 mutex 上进程的最高优先级
    * 但也不是万能的 (例如条件变量唤醒)
* 在系统中动态维护资源依赖关系
  * 优先级继承是它的特例
* 避免高/低优先级的任务争抢资源
  * 对潜在的优先级反转进行预警 (lockdep)
  * TX-based: 冲突的 TX 发生时, 总是低优先级的 abort

#### 多处理器调度的困难所在
既不能简单地 "分配线程到处理器"
  * 线程退出, 瞬间处理器开始围观
也不能简单地 "谁空丢给谁"
* 在**处理器之间迁移会导致 cache/TLB 全都白给**

多处理器调度的两难境地
* 迁移? 可能过一会儿还得移回来
* 不迁移? 造成处理器的浪费

#### 实际情况 (1): 多用户、多任务
A 和 B 要在服务器上跑实验
* A 要跑一个任务, 因为要调用一个库, 只能单线程跑
* B 跑并行的任务, 创建 1000 个线程跑 (公平的调度器 CFS: 公平的把 1000 个线程分到 100 个 cpu 上, B 每个 cpu 分 10 个线程, 而 A 只有一个线程, 只能分到 1 个 cpu 的 1/10)
  * B 获得几乎 100% 的 CPU

于是 Linux 就有了 Linux Namespaces Control Groups (cgroups)
`namespaces (7), cgroups (7)`
cgroup 允许以进程组为单位管理资源
![以进程组为单位管理资源](https://jyywiki.cn/pages/OS/img/cgroups.jpg)

#### 实际情况 (2): Big.LITTLE/能耗
软件可以配置 CPU 的工作模式
  * 开/关/工作频率 (频率越低, 能效越好)
  * 如何在给定功率下平衡延迟 v.s. 吞吐量?

#### 实际情况 (3): Non-Uniform Memory Access
共享内存只是假象
  * L1 Cache 花了巨大的代价才让你感到内存是共享的
  * Producer/Consumer 位于同一个/不同 module 性能差距可能很大

#### 程序执行比你想象得复杂
例子: more CPU time, more progress?
例子`os/concurrency/sum-atomic.c` 就可以 challenge 这一点
```bash
time taskset -c 0   ./a.out
time taskset -c 0,1 ./a.out
```
分配了 1/2 的处理器资源, 反而速度更快了.