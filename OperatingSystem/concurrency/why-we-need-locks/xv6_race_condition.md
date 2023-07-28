# 在 xv6 中创建一个 race condition
[kalloc.c 文件中的 kfree 函数](https://github.com/mit-pdos/xv6-riscv/blob/riscv//kernel/kalloc.c#L47)会将释放的 page 保存于 freelist 中. 从函数中可以看出, 这里有一个锁 kmem.lock, 在加锁的区间内更新了freelist.

1. 现在 comment 锁的 acquire 和 release, 这样原来在上锁区间内的代码就不再受锁保护 (不再是原子执行的).
2. `make qemu`
3. 运行 `usertest`
```sh
xv6 kernel is booting

hart 2 starting
hart 1 starting
init: starting sh
$ usertests
...
test dirtest: OK
test exectest: OK
test pipe1: OK
test killstatus: OK
test preempt: kill... wait... OK
test exitwait: OK
test reparent: scause 0x000000000000000d
sepc=0x0000000080000ae6 stval=0x0505050505050505
panic: kerneltrap
```
可以看到已经有panic了, 所以的确有一些 race condition 触发了 panic (race condition可以有不同的表现形式, 它可能发生, 也可能不发生, 但是在 usertests中显r然是发生了什么).

