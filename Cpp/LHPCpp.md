# Learn How to Program with C++

**How a text becomes an Executable?**
1. The compiler takes source code and transforms into a different format.
2. If there are any error messages while conversion, the compiler shows them to us
3. The output file is generally called an object file
4.  When there are multiple source files, each is compiled, then the object files are linked together to create an executable file (exe)
5. So, the 3 times which are important for a cpp program are:
    1. Compile time
    2. Link time
    3. Runtime


**Smallest cpp app**
```
int main()
{
    return 0;
}
```
1. Some things about cpp:
    1. It is case-sensitive
    2. Uses curly brackets
    3. `;` at end of most lines
2. Not all applications are main()
3. Compile with `clang <>.cpp`
4. It would create a file `a.out`.
5. Now, run it with `./a.out`
6. So, fist the code is compiled.
7. If that succeeds, then the compiler output is linked

**Stream I/O**
1. `>>` sends something to a stream
2. `<<` reads something from a stream
3. `#include <iostream>` for importing the iostream library
4. `std::cout` means console output.
5. So, for printing something to console output : `std::cout << “Hello"`