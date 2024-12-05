#include <stdio.h>
#include "vec.h"

#define bool int
#define true 1
#define false 0

#define num_rows 140

int main()
{
    FILE *fptr;

    // Open a file in read mode
    fptr = fopen("../inputs/day4.in", "r");

    // Early exit if the file does not exist
    if (fptr == NULL)
    {
        perror("Not able to open the file.\n");
        return 1;
    }

    int part1 = 0;
    int part2 = 0;

    char *table[num_rows];

    for (size_t i = 0; i < num_rows; i++)
    {
        char *line = NULL;
        size_t line_bufer_len = 0;
        int len = getline(&line, &line_bufer_len, fptr);
        if (len < 0)
            break;
        table[i] = line;
    }

    size_t num_cols = strlen(table[0]);

    char buff[4];

    for (size_t r = 0; r < num_rows; r++)
    {
        for (size_t c = 0; c < num_cols; c++)
        {
            // printf("r: %d, c: %d\n", r, c);
            // horizontal
            if (c + 3 < num_cols)
            {
                buff[0] = table[r][c];
                buff[1] = table[r][c + 1];
                buff[2] = table[r][c + 2];
                buff[3] = table[r][c + 3];
                if (strcmp(buff, "XMAS") == 0)
                {
                    part1 += 1;
                }
            }
            // horizontal back
            if (c + 3 < num_cols)
            {
                buff[3] = table[r][c];
                buff[2] = table[r][c + 1];
                buff[1] = table[r][c + 2];
                buff[0] = table[r][c + 3];
                if (strcmp(buff, "XMAS") == 0)
                {
                    part1 += 1;
                }
            }
            // vertical
            if (r + 3 < num_rows)
            {
                buff[0] = table[r][c];
                buff[1] = table[r + 1][c];
                buff[2] = table[r + 2][c];
                buff[3] = table[r + 3][c];
                if (strcmp(buff, "XMAS") == 0)
                {
                    part1 += 1;
                }
            }
            // vertical back
            if (r + 3 < num_rows)
            {
                buff[3] = table[r][c];
                buff[2] = table[r + 1][c];
                buff[1] = table[r + 2][c];
                buff[0] = table[r + 3][c];
                if (strcmp(buff, "XMAS") == 0)
                {
                    part1 += 1;
                }
            }
            // diagonal top right
            if (r + 3 < num_rows && c + 3 < num_cols)
            {
                buff[0] = table[r][c];
                buff[1] = table[r + 1][c + 1];
                buff[2] = table[r + 2][c + 2];
                buff[3] = table[r + 3][c + 3];
                if (strcmp(buff, "XMAS") == 0)
                {
                    part1 += 1;
                }
            }
            // diagonal bot right
            if (r + 3 < num_rows && c + 3 < num_cols)
            {
                buff[0] = table[r + 3][c + 0];
                buff[1] = table[r + 2][c + 1];
                buff[2] = table[r + 1][c + 2];
                buff[3] = table[r + 0][c + 3];
                if (strcmp(buff, "XMAS") == 0)
                {
                    part1 += 1;
                }
            }
            // diagonal top left
            if (r + 3 < num_rows && c + 3 < num_cols)
            {
                buff[0] = table[r + 0][c + 3];
                buff[1] = table[r + 1][c + 2];
                buff[2] = table[r + 2][c + 1];
                buff[3] = table[r + 3][c + 0];
                if (strcmp(buff, "XMAS") == 0)
                {
                    part1 += 1;
                }
            }
            // diagonal bot right
            if (r + 3 < num_rows && c + 3 < num_cols)
            {
                buff[0] = table[r + 3][c + 3];
                buff[1] = table[r + 2][c + 2];
                buff[2] = table[r + 1][c + 1];
                buff[3] = table[r + 0][c + 0];
                if (strcmp(buff, "XMAS") == 0)
                {
                    part1 += 1;
                }
            }
            // printf("\n");
        }
    }

    printf("%d\n", part1);
    printf("%d\n", part2);

    // clean up
    for (size_t i = 0; i < num_rows; i++)
    {
        free(table[i]);
    }

    fclose(fptr);

    return 0;
}