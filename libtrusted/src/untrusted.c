#include <assert.h>
#include <stdio.h>
#include <time.h>
#include <unistd.h>

inline void trusted_nsleep(unsigned long ns) {
  if (ns == 0)
    return;
  struct timespec tspec = {0, ns};
  nanosleep(&tspec, NULL);
}

void trusted_msleep(unsigned ms) {
  if (ms == 0)
    return;
  usleep(ms * 1000);
}

void trusted_do_nothing(int *ptr) {
  if (ptr) {
  }
}

void trusted_read_i32(int *ptr) {
  if (ptr) {
    assert(*ptr > 1);
  }
}

void trusted_access_buf(int *ptr, unsigned len) {
  if (!ptr) {
    printf("Error: ptr is nullptr\n");
    return;
  }

  for (int i = 0; (unsigned)i < len; ++i) {
    if (i != ptr[i])
      printf("Error: Buffer initialized improperly: *ptr = %d\n", ptr[i]);
  }
}

void trusted_do_nothing_sleep(int *ptr, unsigned time) {
  trusted_nsleep(time);
  trusted_do_nothing(ptr);
}

void trusted_read_i32_sleep(int *ptr, unsigned time) {
  trusted_nsleep(time);
  trusted_read_i32(ptr);
}

void trusted_access_buf_sleep(int *ptr, unsigned len, unsigned time) {
  trusted_nsleep(time);
  trusted_access_buf(ptr, len);
}

void trusted_callback(int *ptr, unsigned time, void (*f)(int *, unsigned)) {
  trusted_nsleep(time);
  (*f)(ptr, time);
}

void trusted_access_vec(int *ptr, unsigned len, unsigned time) {
  trusted_nsleep(time);
  unsigned i;
  for (i = 0; i < len; ++i) {
    assert(ptr[i%10] == (int)i%10);
  }
}

void trusted_access_box_vec(int **ptr, unsigned len, unsigned time) {
  trusted_nsleep(time);
  unsigned i;
  for (i = 0; i < len; ++i) {
    assert(*ptr[i] == (int)i);
  }
}

