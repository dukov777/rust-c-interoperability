#include <stdio.h>

extern int my_rust_function(int x);
extern int c_function(int a);

int main()
{
    printf("Calling C function: %d\n", c_function(5));
    my_rust_function(5);
    printf("Calling Rust function: %d\n", my_rust_function(5));
    return 0;
}
