#include <assert.h>
#include <stdio.h>
#include <unistd.h>

void msleep(unsigned ms) {
  if (ms == 0)
    return;
  usleep(ms * 1000);
}

void do_nothing(int *ptr) {}

void read_i32(int *ptr) {
  if (ptr) {
    assert(*ptr > 1);
    printf("*ptr = %d\n", *ptr);
  }
}

void access_buf(int *ptr, unsigned len) {
  if (!ptr) {
    printf("Error: ptr is nullptr\n");
    return;
  }

  for (int i = 0; i < len; ++i) {
    if (i != ptr[i])
      printf("Error: Buffer initialized improperly: *ptr = %d\n", ptr[i]);
  }
}

void do_nothing_sleep(int *ptr, unsigned time) { usleep(time); }

void read_i32_sleep(int *ptr, unsigned time) {
  usleep(time);
  read_i32(ptr);
}

void access_buf_sleep(int *ptr, unsigned len, unsigned time) {
  usleep(time);
  access_buf(ptr, len);
}
