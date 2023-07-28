#include "../thread.h"
#include "../thread-sync.h"

int n, count = 0;
mutex_t lk = MUTEX_INIT();
cond_t cv = COND_INIT();

void Tproduce() {
  while (1) {
    mutex_lock(&lk);
    if (count == n) {
      cond_wait(&cv, &lk);  // 我们希望在条件不成立的时候
                            // 把锁释放掉去睡觉
                            // 直到条件满足的时候被唤醒
    }
    printf("("); count++;
    cond_signal(&cv);
    mutex_unlock(&lk);
  }
}

void Tconsume() {
  while (1) {
    mutex_lock(&lk);
    if (count == 0) {
      pthread_cond_wait(&cv, &lk);
    }
    printf(")"); count--;
    cond_signal(&cv);
    mutex_unlock(&lk);
  }
}

int main(int argc, char *argv[]) {
  assert(argc == 2);
  n = atoi(argv[1]);
  setbuf(stdout, NULL);
  create(Tproduce);
  create(Tconsume);
  create(Tconsume);
  // 当有多个生产者和消费者的时候, buggy
  // 实际上一个 producer 两个 consumer 表现就不正常了
  // 因为我们只有一个条件变量, 导致了 consumer 唤醒了 consumer
  //
  // Oops!
  // for (int i = 0; i < 8; i++) { 
  //   create(Tproduce);
  //   create(Tconsume);
  // }
}
