**How Other Languages Handle Memory Part 1 (C)**

The C programming language was created by Brian Kernighan and Dennis Ritchie in the early 1970s.

It was also the first language I learned (in the mid 80's).

To create a programme you can run, you compile a collection of source files into object files, and then link them with libraries; these may be the "standard" libraries or they could be external libraries from a third party.

Once you've compiled your programme into an executable, that code runs natively on your machine. In other words, you're coding at a low level and any checking in your programme needs to be implemented either by you or the people who created the libraries.

As a reminder of the rules of FizzBuzz:
- If the number is a multiple of 3, return "fizz"
- If the number is a multiple of 5, return "buzz"
- Otherwise, return the number (as a string)

Here is an example implementation of FizzBuzz in C.

`fizzbuzz.c`

```
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
  printf("1 [%s] 2 [%s] 3 [%s]\n", result1, result2, result3);
}
```

If you create this file and run it, you'll get fizz and some indecipherable characters in the second and third case. Why? 

At the moment, our function is returning a pointer to a memory location (where we would ideally like to put our return result) that hasn't been initialised.

How could we do that?

We could allocate an array in the function. Let's make that change, and allocate an array within the function.

```
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
  char ret[20];
  sprintf(ret, "%llu", n);
  return &ret[0];
}

int main() {
  char *result1 = fizzbuzz(123456789);
  char *result2 = fizzbuzz(12387812162387);
  char *result3 = fizzbuzz(12345689);
  printf("1 [%s] 2 [%s] 3[%s]\n", result1, result2, result3);
}
```

This still doesn't work. 

The problem is that the local variables in a function are created on the _stack_ and those variables are reclaimed when the function returns. 

This definition

```
char ret[20];
```

declared a 20 character array that is allocted on the stack, so it is reclaimed once we return from fizzbuzz().

That isn't going to work.

**Solution 1**

A possible solution is to allocate some storage in the calling function that we can use for our return string.

```
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
  char buf[20];
  char *result1 = fizzbuzz(buf, 123456789);
  char *result2 = fizzbuzz(buf, 12387812162387);
  char *result3 = fizzbuzz(buf, 12345689);
  printf("1 [%s] 2 [%s] 3 [%s]\n", result1, result2, result3);
}
```

Here we allocate the array (or _buffer_) in the calling function (main) and pass it to `fizzbuff()` to use for building the numeric string result. 

However, the result looks like this:

```
$ ./a.out
1 [fizz] 2 [12345689] 3 [12345689]
$
``` 

(If you don't specify an output file name for your programme, the C compiler will default to `a.out`.)

Since the calls are sharing the same buffer the last call overwrites the second.

We can fix this:

```
// . . .
int main() {
  char buf1[20], buf2[20], buf3[20];
  char *result1 = fizzbuzz(buf1, 123456789);
  char *result2 = fizzbuzz(buf2, 12387812162387);
  char *result3 = fizzbuzz(buf3, 12345689);
  printf("1 [%s] 2 [%s] 3 [%s]\n", result1, result2, result3);
}
```

How do we know what size to make these arrays?

Well, first of all let's notice that they are fixed size. When we convert a numeric result to a string, `sprintf` is copying the characters of a string into the buffer we provide. We need to ensure that it's big enough. 

Using the built-in numeric types, we can represent numbers of type `unsigned long long` with a maximum value of 18,446,744,073,709,551,615 (20 digits). So really our array should be 21, because C strings always have a trailing null character to indicate end of string.

We could support numbers larger than that, though (using arrays of native integers).

So if we're using fixed arrays, we must ensure that our function doesn't try to populate them with strings that are larger than expected; this can be done by choosing sizes that are big enough for all cases, possibly with argument checking by the function.

In other cases, where you're working with values based on external input, you have to be very careful that your buffer sizes are not exceeded.

**Solution 2**

We can also allocate the memory dynamically.

```
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
  char *buf = malloc(sizeof(char)*(numDigits(n)+1));
  sprintf(buf, "%llu", n);
  return buf;
}

int main() {
  char *result1 = fizzbuzz(123456789);
  char *result2 = fizzbuzz(12387812162387);
  char *result3 = fizzbuzz(12345689);
  printf("1 [%s] 2 [%s] 3 [%s]\n", result1, result2, result3);
}
```

The `malloc` function allocates memory for the heap.

This memory stays allocated until either the programme exits, or the memory is freed. 

This programme ends pretty quickly, but for a long-running programme calls to malloc that don't have a corresponding `free` could eventually lead to the programme running out of heap space.

So we could call `free` on the return result. But that wouldn't always work, since we're returning non-alloc'd strings too.

Allocating the array buffer in the calling function has the advantage of simplicity, as you're able to determine in advance the maximum size required. Otherwise consider allocating memory using malloc, and structure your programme so that you can match a malloc with a free. If you're programme is never going to run for very long you can overlook this cleanup, but it isn't good practice (you could be leaving hidden issues for unsuspecting future colleagues).

Exercises:

1. What are the two types of memory area from which space for variables is typically allocated in C?

[details="Answer"]
Heap, and stack.
[/details]

@london3 @wintreese  @Beaver 
