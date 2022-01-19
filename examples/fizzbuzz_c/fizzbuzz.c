#include <stdio.h>
#include <stdlib.h>

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
  return ret; 
}

int main() {
  puts(fizzbuzz(15));
  puts(fizzbuzz(123456789));
  puts(fizzbuzz(12387812162387));
}
