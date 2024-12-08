#include <stdio.h>
#include <string.h>
#include "vec.h"

#define bool int
#define true 1
#define false 0

// directions
#define up 0
#define down 1
#define left 2
#define right 3

#define num_rows 130

reset_table(char **table, size_t width, size_t height)
{
    for (size_t r = 0; r < width; r++)
    {
        for (size_t c = 0; c < height; c++)
        {
            if (table[r][c] != '#')
            {
                table[r][c] = '.';
            }
        }
    }
}

print_table(char **table, size_t width, size_t height)
{
    // for (size_t r = 0; r < width; r++)
    // {
    //     for (size_t c = 0; c < height; c++)
    //     {
    //         printf("%c", table[r][c]);
    //     }
    //     printf("\n");
    // }
    // printf("\n");
}

bool walk(char **table, size_t width, size_t height, size_t start_c, size_t start_r, int *steps)
{
    bool done = false;
    int direction = up;
    size_t gr = start_r;
    size_t gc = start_c;

    bool loop = false;

    while (!done)
    {
        // table[gr][gc] = 'X';
        switch (direction)
        {
        case up:
            if (gr == 0)
            {
                done = true;
            }
            else
            {
                if (table[gr - 1][gc] == '#')
                {
                    direction = right;
                }
                else
                {
                    if (table[gr][gc] == 'U')
                    {
                        loop = true;
                        return loop;
                    }
                    if (table[gr][gc] == '.')
                    {
                        *steps += 1;
                    }
                    table[gr][gc] = 'U';
                    gr -= 1;
                }
            }
            break;
        case down:
            if (gr == height - 1)
            {
                done = true;
            }
            else
            {
                if (table[gr + 1][gc] == '#')
                {
                    direction = left;
                }
                else
                {
                    if (table[gr][gc] == 'D')
                    {
                        loop = true;
                        return loop;
                    }
                    if (table[gr][gc] == '.')
                    {
                        *steps += 1;
                    }
                    table[gr][gc] = 'D';
                    gr += 1;
                }
            }
            break;
        case left:
            if (gc == 0)
            {
                done = true;
            }
            else
            {
                if (table[gr][gc - 1] == '#')
                {
                    direction = up;
                }
                else
                {
                    if (table[gr][gc] == 'L')
                    {
                        loop = true;
                        return loop;
                    }
                    if (table[gr][gc] == '.')
                    {
                        *steps += 1;
                    }
                    table[gr][gc] = 'L';
                    gc -= 1;
                }
            }
            break;
        case right:
            if (gc == width - 1)
            {
                done = true;
            }
            else
            {
                if (table[gr][gc + 1] == '#')
                {
                    direction = down;
                }
                else
                {
                    if (table[gr][gc] == 'R')
                    {
                        loop = true;
                        return loop;
                    }
                    if (table[gr][gc] == '.')
                    {
                        *steps += 1;
                    }
                    table[gr][gc] = 'R';
                    gc += 1;
                }
            }
            break;

        default:
            break;
        }
    }
    table[gr][gc] = 'E';

    *steps += 1;
    return loop;
}

int main()
{
    FILE *fptr;

    // Open a file in read mode
    fptr = fopen("../inputs/day6.in", "r");

    // Early exit if the file does not exist
    if (fptr == NULL)
    {
        perror("Not able to open the file.\n");
        return 1;
    }

    int part1 = 1;

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

    // endline is included
    size_t num_cols = strlen(table[0]) - 1;

    char buff[4];

    printf("num_rows: %ld\n", num_rows);
    printf("num_cols: %ld\n", num_cols);

    size_t start_r;
    size_t start_c;

    for (size_t r = 0; r < num_rows; r++)
    {
        for (size_t c = 0; c < num_cols; c++)
        {
            if (table[r][c] == '^')
            {
                start_r = r;
                start_c = c;
            }
        }
        printf("\n");
    }
    printf("\n");

    int loop = walk(table, num_cols, num_rows, start_c, start_r, &part1);
    printf("loop: %d\n", loop);
    printf("%d\n", part1);

    print_table(table, num_cols, num_rows);

    int part2 = 0;

    reset_table(table, num_cols, num_rows);
    print_table(table, num_cols, num_rows);

    // try to add extra obstacle to every position
    for (size_t r = 0; r < num_rows; r++)
    {
        for (size_t c = 0; c < num_cols; c++)
        {
            if (table[r][c] != '.' || (r == start_r && c == start_c))
            {
                continue;
            }
            table[r][c] = '#';

            loop = walk(table, num_cols, num_rows, start_c, start_r, &part1);
            part2 += loop;
            if (loop)
            {
                // printf("r: %d, c: %d\n", r, c);
                print_table(table, num_cols, num_rows);
            }
            reset_table(table, num_cols, num_rows);
            table[r][c] = '.';
        }
    }

    printf("%d\n", part2);

    // clean up
    for (size_t i = 0; i < num_rows; i++)
    {
        free(table[i]);
    }

    fclose(fptr);

    return 0;
}