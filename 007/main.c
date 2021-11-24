#include <stdio.h>

int is_prime(x) {
    int is_prime = 1;
    int n = x;
    for (int d = 2; d < n; d++) {
        if (x%d == 0) {
            is_prime = 0;
            break;
        }
    }
    return is_prime;
}

int main() {
    int result;
    int x = 2;
    int count = 0;
    int n = 10001;
    while (count < n) {
        result = is_prime(x);
        if (result) {
            count++;
            printf("%d\n", x);
        }
        x++;
    }
}
