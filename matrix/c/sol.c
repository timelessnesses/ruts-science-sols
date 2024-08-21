/**
 * I hate C so much yet people using it
 */

#include <stdio.h>

void setup_matrix(int row, int col, int matrix[row][col]) {
    for (int i = 0; i < row; i++) {
        for (int j = 0; j < col; j++) {
            printf("Enter number [%d][%d]: ", i + 1, j + 1);
            scanf("%d", &matrix[i][j]);
        }
    }
}

int main() {
    int M, N;
    printf("Rows: ");
    scanf("%d", &M);
    printf("Columns: ");
    scanf("%d", &N);

    int first_matrix[M][N], second_matrix[M][N], result[M][N];

    printf("Enter first matrix values (%dx%d)\n", M, N);
    setup_matrix(M, N, first_matrix);

    printf("Enter second matrix values (%dx%d)\n", M, N);
    setup_matrix(M, N, second_matrix);

    for (int i = 0; i < M; i++) {
        for (int j = 0; j < N; j++) {
            result[i][j] = first_matrix[i][j] + second_matrix[i][j];
        }
    }

    printf("Result:\n");
    for (int i = 0; i < M; i++) {
        for (int j = 0; j < N; j++) {
            printf("%d\t", result[i][j]);
        }
        printf("\n");
    }

    return 0;
}
