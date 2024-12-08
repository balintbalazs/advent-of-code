#include <stdio.h>
#include <string.h>
#include "vec.h"

#define bool int
#define true 1
#define false 0

#define num_rows 50

int main()
{
    FILE *fptr;

    // Open a file in read mode
    fptr = fopen("../inputs/day8.in", "r");

    // Early exit if the file does not exist
    if (fptr == NULL)
    {
        perror("Not able to open the file.\n");
        return 1;
    }

    int part1 = 0;
    int part2 = 0;

    char *map[num_rows];

    for (size_t i = 0; i < num_rows; i++)
    {
        char *line = NULL;
        size_t line_bufer_len = 0;
        int len = getline(&line, &line_bufer_len, fptr);
        if (len < 0)
            break;
        map[i] = line;
    }

    fclose(fptr);

    Vec antennas[128];
    for (size_t i = 0; i < 128; i++)
    {
        antennas[i] = create_vec(20);
    }

    Vec frequencies = create_vec(128);

    // ignore endline
    size_t num_cols = strlen(map[0]) - 1;

    for (size_t r = 0; r < num_rows; r++)
    {
        for (size_t c = 0; c < num_cols; c++)
        {
            if (map[r][c] != '.')
            {
                bool found = sorted_vec_contains(&frequencies, map[r][c]);
                if (!found)
                {
                    push_vec(&frequencies, map[r][c]);
                    sort_vec(&frequencies);
                }
                push_vec(&antennas[map[r][c]], r);
                push_vec(&antennas[map[r][c]], c);
                // remove antennas from map so ovelapping antinodes can be put on map
                map[r][c] = '.';
            }
        }
    }

    for (size_t i = 0; i < frequencies.size; i++)
    {
        printf("%c\n", (char)frequencies.head[i]);
        int f = frequencies.head[i];
        for (size_t a1 = 0; a1 < antennas[f].size / 2 - 1; a1++)
        {
            for (size_t a2 = a1 + 1; a2 < antennas[f].size / 2; a2++)
            {
                int r1 = antennas[f].head[a1 * 2];
                int c1 = antennas[f].head[a1 * 2 + 1];
                int r2 = antennas[f].head[a2 * 2];
                int c2 = antennas[f].head[a2 * 2 + 1];

                // possible antinode coordinates
                int r_an1 = r1 + (r1 - r2);
                int c_an1 = c1 + (c1 - c2);
                int r_an2 = r2 + (r2 - r1);
                int c_an2 = c2 + (c2 - c1);

                if (r_an1 >= 0 && r_an1 < num_rows && c_an1 >= 0 && c_an1 < num_cols)
                {
                    if (map[r_an1][c_an1] == '.')
                    {
                        map[r_an1][c_an1] = '#';
                        part1 += 1;
                    }
                }
                if (r_an2 >= 0 && r_an2 < num_rows && c_an2 >= 0 && c_an2 < num_cols)
                {
                    if (map[r_an2][c_an2] == '.')
                    {
                        map[r_an2][c_an2] = '#';
                        part1 += 1;
                    }
                }
            }
        }
    }

    // part2
    // clear map

    for (size_t r = 0; r < num_rows; r++)
    {
        for (size_t c = 0; c < num_cols; c++)
        {
            map[r][c] = '.';
        }
    }

    for (size_t i = 0; i < frequencies.size; i++)
    {
        printf("%c\n", (char)frequencies.head[i]);
        int f = frequencies.head[i];
        for (size_t a1 = 0; a1 < antennas[f].size / 2 - 1; a1++)
        {
            for (size_t a2 = a1 + 1; a2 < antennas[f].size / 2; a2++)
            {
                int r1 = antennas[f].head[a1 * 2];
                int c1 = antennas[f].head[a1 * 2 + 1];
                int r2 = antennas[f].head[a2 * 2];
                int c2 = antennas[f].head[a2 * 2 + 1];

                // possible antinode coordinates
                int r_diff = r2 - r1;
                int c_diff = c2 - c1;

                for (size_t m = 0;; m++)
                {
                    int r_a = r2 + m * r_diff;
                    int c_a = c2 + m * c_diff;
                    if (r_a >= 0 && r_a < num_rows && c_a >= 0 && c_a < num_cols)
                    {
                        if (map[r_a][c_a] == '.')
                        {
                            map[r_a][c_a] = '#';
                            part2 += 1;
                        }
                    } else {
                        break;
                    }
                }
                for (size_t m = -1;; m--)
                {
                    int r_a = r2 + m * r_diff;
                    int c_a = c2 + m * c_diff;
                    if (r_a >= 0 && r_a < num_rows && c_a >= 0 && c_a < num_cols)
                    {
                        if (map[r_a][c_a] == '.')
                        {
                            map[r_a][c_a] = '#';
                            part2 += 1;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    printf("part1: %d\n", part1);
    printf("part2: %d\n", part2);

    // clean up
    for (size_t i = 0; i < num_rows; i++)
    {
        free(map[i]);
    }
    for (size_t i = 0; i < 128; i++)
    {
        free_vec(antennas[i]);
    }
    free_vec(frequencies);

    return 0;
}