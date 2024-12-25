#include <stdio.h>

extern int rust_function(int x);
extern int c_function(int a);

int main()
{
    printf("Calling C function: %d\n", c_function(5));
    rust_function(5);
    printf("Calling Rust function: %d\n", rust_function(5));
    return 0;
}
