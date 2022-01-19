#include <stdio.h>

char *fizzbuzz(unsigned long long n) {
  if (n % 3 == 0 && n % 5 == 0) {
    return "fizzbuzz";
  }
  if (n % 3 == 0) {
    return "fizz";
  }
  if (n % 5 == 0) {
    return "buzz";
  }
  char *ret;
  return ret; 
}

int main() {
  char *result1 = fizzbuzz(123456789);
  char *result2 = fizzbuzz(12387812162387);
  char *result3 = fizzbuzz(12345689);
  printf("1 [%s] 2 [%s] 3[%s]\n", result1, result2, result3);
}
