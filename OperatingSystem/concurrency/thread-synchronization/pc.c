// 生产者-消费者问题:
// 括号问题: 使得打印的括号序列满足
// 1. 一定是某个合法括号序列的前缀
// 2. 括号嵌套的深度不超过 n
//    n=3, ((())())((( 合法
//    n=3, (((()))), (())) 不合法
// 3. 同步
//    等到有空位再打印左括号
//    等到能配对时再打印右括号
// 左括号: 生产资源 (任务)、放入队列
// 右括号: 从队列取出资源 (任务) 执行
// 
// 方案一: 用互斥锁保持条件成立
// 
// follow up: 虽然互斥锁不会 spinning,
// 但是它还是会不断的醒来, goto retry
// 某种意义上还是在自旋
#include "../thread.h"
#include "../thread-sync.h"

int n, count = 0;
mutex_t lk = MUTEX_INIT();

void Tproduce() {
  while (1) {
retry:                    // 同步问题的共性 (都可写成)
    mutex_lock(&lk);      // 先上一把锁
    if (!(count != n)) {     // 判断条件是否成立
      mutex_unlock(&lk);  // 如果条件不成立 (释放锁)
      goto retry;         // 再等等
    }
    count++;
    printf("(");
    mutex_unlock(&lk);
  }
}

void Tconsume() {
  while (1) {
retry:
    mutex_lock(&lk);
    if (!(count != 0)) {
      mutex_unlock(&lk);
      goto retry;
    }
    count--;
    printf(")");
    mutex_unlock(&lk);
  }
}

int main(int argc, char *argv[]) {
  assert(argc == 2);
  n = atoi(argv[1]);
  setbuf(stdout, NULL);

  for (int i = 0; i < 8; i++) {
    create(Tproduce);
    create(Tconsume);
  }
}
