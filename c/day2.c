#include <stdio.h>
#include <ctype.h>
#include <string.h>
#include "vec.h"

#define bool int
#define true 1
#define false 0

int main()
{
    FILE *fptr;

    // Open a file in read mode
    fptr = fopen("../inputs/day2.in", "r");

    // Early exit if the file does not exist
    if (fptr == NULL)
    {
        perror("Not able to open the file.\n");
        return 1;
    }

    char *line = NULL;
    size_t line_bufer_len = 0;
    size_t line_len = 0;

    int part1 = 0;
    int part2 = 0;
    while (getline(&line, &line_bufer_len, fptr) != -1)
    {
        line_len = strlen(line);

        long levels[20] = {0};
        char *end = line;
        size_t level_index = 0;
        while (end < line + line_len - 1)
        {
            levels[level_index] = strtol(end, &end, 10);
            level_index += 1;
        }

        // print for debug
        for (size_t i = 0; i < level_index; i++)
        {
            printf("%ld ", levels[i]);
        }

        bool increasing = levels[0] < levels[1];
        printf("%d\t", increasing);

        bool safe = true;
        int bad_levels = 0;
        for (size_t i = 1; i < level_index; i++)
        {
            if (increasing && levels[i - 1] > levels[i])
            {
                printf("\tnot increasing all the time");
                safe = false;
                bad_levels += 1;
            }
            if (!increasing && levels[i - 1] < levels[i])
            {
                printf("\tnot decreasing all the time, %ld", i);
                safe = false;
                bad_levels += 1;
            }
            int diff = abs(levels[i] - levels[i - 1]);
            if (diff < 1 || diff > 3)
            {
                printf("\ttoo much diff %d", diff);
                safe = false;
                bad_levels += 1;
            }
        }
        printf("\t");
        if (safe)
        {
            part1 += 1;
        }
        if (bad_levels <= 1) {
            part2 += 1;
        }

        printf("\n");
    }

    printf("%d\n", part1);
    printf("%d\n", part2);

    // clean up
    free(line);
    fclose(fptr);

    return 0;
}