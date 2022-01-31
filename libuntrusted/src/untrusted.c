#include <assert.h>
#include <stdio.h>
#include <time.h>
#include <unistd.h>

inline void nsleep(unsigned long ns) {
  if (ns == 0)
    return;
  struct timespec tspec = {0, ns};
  nanosleep(&tspec, NULL);
}

void msleep(unsigned ms) {
  if (ms == 0)
    return;
  usleep(ms * 1000);
}

void do_nothing(int *ptr) {
  if (ptr) {
  }
}

void read_i32(int *ptr) {
  if (ptr) {
    assert(*ptr > 1);
    /*printf("*ptr = %d\n", *ptr);*/
  }
}

void access_buf(int *ptr, unsigned len) {
  if (!ptr) {
    printf("Error: ptr is nullptr\n");
    return;
  }

  for (int i = 0; (unsigned)i < len; ++i) {
    if (i != ptr[i])
      printf("Error: Buffer initialized improperly: *ptr = %d\n", ptr[i]);
  }
}

void do_nothing_sleep(int *ptr, unsigned time) {
  nsleep(time);
  do_nothing(ptr);
}

void read_i32_sleep(int *ptr, unsigned time) {
  nsleep(time);
  read_i32(ptr);
}

void access_buf_sleep(int *ptr, unsigned len, unsigned time) {
  nsleep(time);
  access_buf(ptr, len);
}

void callback(int *ptr, unsigned time, void (*f)(int *, unsigned)) {
  nsleep(time);
  (*f)(ptr, time);
}

void access_vec(int *ptr, unsigned len, unsigned time) {
  nsleep(time);
  unsigned i;
  for (i = 0; i < len; ++i) {
    assert(ptr[i%10] == (int)i%10);
  }
}

void access_box_vec(int **ptr, unsigned len, unsigned time) {
  nsleep(time);
  unsigned i;
  for (i = 0; i < len; ++i) {
    assert(*ptr[i] == (int)i);
  }
}
