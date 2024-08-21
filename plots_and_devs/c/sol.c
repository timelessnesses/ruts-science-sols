#include <stdio.h>
#include <stdlib.h>

int clamp(int n, int min_, int max_) {
    if (n > max_) return max_;
    if (n < min_) return min_;
    return n;
}

int main() {
    int N, M;
    printf("Plots: ");
    scanf("%d", &N);
    printf("Developments: ");
    scanf("%d", &M);

    int *BUILDINGS = (int *)malloc(N * sizeof(int));
    for (int i = 0; i < N; i++) {
        BUILDINGS[i] = 0;
    }

    for (int i = 0; i < M; i++) {
        int start, end, height_change;
        printf("Input the start, ending and building height change split by space: ");
        scanf("%d %d %d", &start, &end, &height_change);

        for (int x = start - 1; x < end; x++) {
            BUILDINGS[x] += height_change;
        }

        int max_height = BUILDINGS[0];
        for (int j = 1; j < N; j++) {
            if (BUILDINGS[j] > max_height) {
                max_height = BUILDINGS[j];
            }
        }

        int tallest_near_buildings_together_count = 0;
        for (int j = 0; j < N; j++) {
            if (BUILDINGS[j] == max_height &&
                (BUILDINGS[clamp(j - 1, 0, N - 1)] == max_height || BUILDINGS[clamp(j + 1, 0, N - 1)] == max_height)) {
                tallest_near_buildings_together_count++;
            }
        }

        printf("Max height: %d, Tallest near buildings count: %d\n", max_height, tallest_near_buildings_together_count);
        printf("\n");
    }

    free(BUILDINGS);
    return 0;
}
