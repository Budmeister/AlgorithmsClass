
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

struct Coord {
    int x;
    int y;
};

void get_input(int *n, int *k);
int count_positions(int n, int k);
void count_positions_r(int n, int k, struct Coord start, bool *diag1s, bool *diag2s, int *count);
int diag1(struct Coord *coord);
int diag2(struct Coord *coord);

int main() {
    int n, k;
    while (1) {
        get_input(&n, &k);
        if (n == 0 && k == 0) {
            break;
        }

        int count = count_positions(n, k);
        printf("%d\n", count);
    }
    return 0;
}

void get_input(int *n, int *k) {
    scanf("%d %d", n, k);
}

int count_positions(int n, int k) {
    int diag1s_max = (n - 1) + (n - 1) + 1;
    // Range: [0, 2 * n - 1)
    bool *diag1s = malloc(diag1s_max * sizeof(bool));
    int diag2s_min = (0) - (n - 1);
    int diag2s_max = (n - 1) - 0 + 1;
    bool *diag2s = malloc((diag2s_max - diag2s_min) * sizeof(bool));
    // Range: [-(n-1), n)
    diag2s -= diag2s_min;

    int count = 0;
    struct Coord start;
    start.x = 0;
    start.y = 0;

    count_positions_r(n, k, start, diag1s, diag2s, &count);

    return count;
}

void print_indents(int k) {
    for (int i = 0; i < 4 - k; i++)
        printf("\t");
}

void count_positions_r(int n, int k, struct Coord start, bool *diag1s, bool *diag2s, int *count) {
    // print_indents(k);
    // printf("n: %d, k: %d\n", n, k);
    while (start.x < n) {
        while (start.y < n) {
            // print_indents(k);
            // printf("x: %d, y: %d\n", start.x, start.y);
            int d1 = diag1(&start);
            int d2 = diag2(&start);
            bool is_attacking = diag1s[d1] || diag2s[d2];
            if (!is_attacking) {
                // print_indents(k);
                // printf("Safe (k=%d)", k);
                if (k == 1) {
                    // printf(" - Incrementing count\n");
                    (*count)++;
                } else {
                    // printf("\n");
                    diag1s[d1] = true;
                    diag2s[d2] = true;
                    count_positions_r(n, k-1, start, diag1s, diag2s, count);
                    diag1s[d1] = false;
                    diag2s[d2] = false;
                }
            }
            start.y++;
            // print_indents(k);
            // printf("(x=%d, y=%d, n=%d, k=%d)\n", start.x, start.y, n, k);
        }
        start.x++;
        start.y = 0;
    }
}

int diag1(struct Coord *coord) {
    return coord->x + coord->y;
}

int diag2(struct Coord *coord) {
    return coord->x - coord->y;
}
