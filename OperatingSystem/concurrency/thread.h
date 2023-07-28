#include <stdlib.h>
#include <stdio.h>
#include <assert.h>
#include <pthread.h>

struct thread{   //链表
  int id;//线程id 线程号 从1开始
  pthread_t thread;//POSIX线程库规定的线程号
  void (*entry)(int); //线程的入口地址
  struct thread *next;//指向下一个线程
};

struct thread *threads;  //单链表头
void (*join_fn)();

__attribute__((destructor)) static void join_all(){ //main函数结束之后调用 
  for(struct thread *next;threads;threads=next){   //遍历整个进程链表
    pthread_join(threads->thread,NULL);
    next=threads->next;
    free(threads);
  }
  join_fn?join_fn():(void)0;
}

static inline void *entry_all(void *arg){//符合pthreads_create要求
  struct thread *thread =(struct thread *)arg;//取出线程对象指针cur
  thread->entry(thread->id);//调用该线程的entry，对线程入口赋值
  return NULL;//
}

//线程创建
static inline void create(void *fn){
  struct thread *cur=(struct thread *)malloc(sizeof(struct thread));//分配内存
  assert(cur);//假设内存分配成功
  cur->id=threads?threads->id+1:1;//当钱线程的线程号
  cur->next=threads;
  cur->entry=(void (*)(int))fn;
  threads   =cur;
  pthread_create(&cur->thread,NULL,entry_all,cur);
}

static inline void join(void (*fn)()){  
  join_fn=fn;  //将函数指针赋值给全局变量
}
