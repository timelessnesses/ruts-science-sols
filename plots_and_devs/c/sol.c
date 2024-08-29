#include <stdio.h>
#include <stdlib.h>

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

        int max_near_buildings_count = 0;
        int current_count = 0;

        for (int j = 0; j < N; j++) {
            if (BUILDINGS[j] == max_height) {
                current_count++;
            } else {
                if (current_count > max_near_buildings_count) {
                    max_near_buildings_count = current_count;
                }
                current_count = 0;
            }
        }

        if (current_count > max_near_buildings_count) {
            max_near_buildings_count = current_count;
        }

        printf("Max height: %d, Tallest near buildings count: %d\n", max_height, max_near_buildings_count);
    }

    free(BUILDINGS);
    return 0;
}
