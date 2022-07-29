#include <stdio.h>
#include <stdlib.h>

extern int sum(int, int);

int main(void) {
    int result = sum(5, 3);
    printf("Result: %d\n", result);

    return 0;
}
