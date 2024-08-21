#include <stdio.h>
#include <stdbool.h>

bool prime(int n) {
    if (n <= 1) {
        return false;
    }
    for (int x = 2; x < n; x++) {
        if (n % x == 0) {
            return false;
        }
    }
    return true;
}

bool stupid_timeless_prime(int n) {
    if (n <= 1) {
        return false;
    }
    int divisors[n];
    int index = 0;

    for (int x = 1; x <= n; x++) {
        if (n % x == 0) {
            divisors[index++] = x;
        }
    }

    return index == 2;
}

bool odd(int n) {
    return n % 2 != 0;
}

int main() {
    int N;
    printf("Enter number of times you want to check numbers: ");
    scanf("%d", &N);

    int a[N];

    for (int x = 0; x < N; x++) {
        printf("Enter number %d: ", x + 1);
        scanf("%d", &a[x]);
    }

    printf("Is Prime     |     Is Odd\n");

    for (int x = 0; x < N; x++) {
        bool p = prime(a[x]);
        bool o = odd(a[x]);

        printf("%s         |     %s\n", p ? "true" : "false", o ? "true" : "false");
    }

    return 0;
}
