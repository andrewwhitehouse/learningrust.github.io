#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int numDigits(unsigned long long n) {
  int result = 0;
  while (n > 0) {
    result++;
    n /= 10;
  }
  return result;
}

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
  char *ret = malloc(numDigits(n)+1);
  sprintf(ret, "%llu", n);
  return &ret[0]; 
}

void allocateSomethingElse() {
  char unused[100000];
  memset(unused, 'x', sizeof(unused)/sizeof(char));
}

int main() {
  char *result1 = fizzbuzz(123456789);
  char *result2 = fizzbuzz(12387812162387);
  char *result3 = fizzbuzz(12345689);
  allocateSomethingElse();
  printf("1 [%s] 2 [%s] 3[%s]\n", result1, result2, result3);
}
