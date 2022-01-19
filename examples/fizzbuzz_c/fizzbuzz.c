#include <stdio.h>

char *fizzbuzz(char *buf, unsigned long long n) {
  if (n % 3 == 0 && n % 5 == 0) {
    return "fizzbuzz";
  }
  if (n % 3 == 0) {
    return "fizz";
  }
  if (n % 5 == 0) {
    return "buzz";
  }
  sprintf(buf, "%llu", n);
  return buf;
}

int main() {
  char buf1[20], buf2[20], buf3[20];
  char *result1 = fizzbuzz(buf1, 123456789);
  char *result2 = fizzbuzz(buf2, 12387812162387);
  char *result3 = fizzbuzz(buf3, 12345689);
  printf("1 [%s] 2 [%s] 3 [%s]\n", result1, result2, result3);
}
