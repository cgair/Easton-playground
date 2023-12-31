#include <sys/time.h>
#include <unistd.h>
#include <stdio.h>
#include <time.h>
// 时间: 内核维护秒级的时间 (所有进程映射同一个页面)

double gettime() {
  struct timeval t;
  gettimeofday(&t, NULL); // trapless system call
  return t.tv_sec + t.tv_usec / 1000000.0;
}

int main() {
  printf("Time stamp: %ld\n", time(NULL)); // trapless system call
  double st = gettime();
  sleep(1);
  double ed = gettime();
  printf("Time: %.6lfs\n", ed - st);
}
