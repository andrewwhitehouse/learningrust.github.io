**Garbage Collection**

We'll get back to Rust very shortly, but it's worth reflecting on previous approaches to memory management before we do.

Coding in C and C++ placed the burden of memory management on the developer. 

In 1995 Java was introduced by Sun Microsystems, with C-like syntax, and the promise of "write once, run anywhere" and an approach to memory management known as garbage collection which was first introduced with LISP, in 1959 and been adopted by various languages since then.

Java source files are compiled into an intermediate format, called _class_ files, which are then interpreted by the Java Runtime Environment (JRE). The JRE is very good at optimising long-running code, although initial start times can be higher because the JRE comes with an overhead.

Increasingly sophisticated [approaches](https://www.freecodecamp.org/news/garbage-collection-in-java-what-is-gc-and-how-it-works-in-the-jvm/) have been devised to reduce the amount of work the garbage collector has to do, which may potentially interrupt programme execution.

Runtime objects are garbage collection when references to them are released; so it's important to ensure that object references are not retained unnecessarily (for example with a static HashMap in a class that maintains references to other objects, and it never clears). Objects that can't be released will lead to memory leaks, and processes possibly experiencing OutOfMemory errors.

This small amount of background may help you to appreciate Rust's approach even more.

Copyright (c) 2022 Andrew Whitehouse. All rights reserved.
