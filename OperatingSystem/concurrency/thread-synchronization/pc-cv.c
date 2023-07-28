#include "../thread.h"
#include "../thread-sync.h"

int n, count = 0;
mutex_t lk = MUTEX_INIT();
cond_t cv = COND_INIT();

void Tproduce() {
  while (1) {
    mutex_lock(&lk);
    // if (count == n) {
    while (!(count != n)) {
      cond_wait(&cv, &lk);
    }
    assert(count != n);    // while 退出时可以保证 count != n and lock is held
    printf("("); count++;
    // cond_signal(&cv);
    cond_broadcast(&cv);
    mutex_unlock(&lk);
  }
}

void Tconsume() {
  while (1) {
    mutex_lock(&lk);
    // if (count == 0) {
    while (count == 0) {
      pthread_cond_wait(&cv, &lk);
    }
    printf(")"); count--;
    // cond_signal(&cv);
    cond_broadcast(&cv);
    mutex_unlock(&lk);
  }
}

int main(int argc, char *argv[]) {
  assert(argc == 2);
  n = atoi(argv[1]);
  setbuf(stdout, NULL);
  // for (int i = 0; i < 1; i++) {
  for (int i = 0; i < 8; i++) {  
                              // use while(..) {
                              // use cond_broadcast(..) {
    create(Tproduce);
    create(Tconsume);
  }
}
