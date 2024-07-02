#include <stdio.h>

int main() {
    int num1 = 10;
    int num2 = 20;
    int sum = num1 + num2;
    printf("The sum of %d and %d is %d\n", num1, num2, sum);
    
    int *ptr = NULL; // Create a null pointer
    *ptr = 42; // Dereference the null pointer (this will cause a segmentation fault)
    
    return 0;
}
