#include "../thread.h"
#include "../thread-sync.h"
#include <stdbool.h>

#define N 3

bool avail[N];

// 万能的方法
mutex_t lck = MUTEX_INIT();
cond_t cv = COND_INIT();

void Tphilosopher(int id) {
  int lhs = (N + id - 1) % N;
  int rhs = id % N;
  while (1) {
    mutex_lock(&lck);
    if (!(avail[lhs] && avail[rhs])) {
      cond_wait(&cv, &lck);
    }
    printf("T%d Got %d\n", id, lhs + 1);
    printf("T%d Got %d\n", id, rhs + 1);
    avail[lhs] = avail[rhs] = true;
    mutex_unlock(&lck);

    mutex_lock(&lck);
    avail[lhs] = avail[rhs] = true;
    cond_broadcast(&cv);
    mutex_unlock(&lck);
  }
}

int main(int argc, char *argv[]) {
  for (int i = 0; i < N; i++) {
    avail[i] = true;
  }
  for (int i = 0; i < N; i++) {
    create(Tphilosopher);
  }
}
