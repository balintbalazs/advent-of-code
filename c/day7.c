#include <stdio.h>
#include <string.h>
#include <math.h>

#define bool int
#define true 1
#define false 0

long num_digits(long num)
{
    long digits = 0;
    do
    {
        num = num / 10;
        digits += 1;
    } while (num);
    return digits;
}

long ipow(long base, long exp)
{
    long result = 1;
    while (exp)
    {
        if (exp & 1)
        {
            result *= base;
        }
        exp >>= 1;
        base *= base;
    }
    return result;
}

long eval(long *nums, int count, char *ops)
{
    // printf("Checking\n");
    // for (size_t i = 0; i < count; i++)
    // {
    //     printf("%d", nums[i]);
    //     if (i < count - 1)
    //     {
    //         printf("%c", ops[i]);
    //     }
    // }
    // printf("\n");

    long res = nums[0];
    for (size_t i = 1; i < count; i++)
    {
        switch (ops[i - 1])
        {
        case '+':
            res = res + nums[i];
            break;
        case '*':
            res = res * nums[i];
            break;

        default:
            break;
        }
    }
    // printf("result: %ld\n\n", res);
    return res;
}

long eval2(long *nums, int count, char *ops)
{
    // printf("Checking\n");
    // for (size_t i = 0; i < count; i++)
    // {
    //     printf("%d", nums[i]);
    //     if (i < count - 1)
    //     {
    //         printf("%c", ops[i]);
    //     }
    // }
    // printf("\n");

    long res = nums[0];
    for (size_t i = 1; i < count; i++)
    {
        switch (ops[i - 1])
        {
        case '+':
            res = res + nums[i];
            break;
        case '*':
            res = res * nums[i];
            break;
        case '|':
            res = res * ipow(10, num_digits(nums[i])) + nums[i];
            break;

        default:
            break;
        }
    }
    // printf("result: %ld\n\n", res);
    return res;
}

int main()
{
    FILE *fptr;

    // Open a file in read mode
    fptr = fopen("../inputs/day7.in", "r");

    // Early exit if the file does not exist
    if (fptr == NULL)
    {
        perror("Not able to open the file.\n");
        return 1;
    }

    char *line = NULL;
    size_t line_bufer_len = 0;
    size_t line_len = 0;

    int line_index = 0;

    long part1 = 0;
    long part2 = 0;

    long nums[50];
    char ops[50];

    while (getline(&line, &line_bufer_len, fptr) != -1)
    {
        line_len = strlen(line);

        long target;
        sscanf(line, "%ld:", &target);
        // number, ':', space
        int offset = num_digits(target) + 2;

        long buff;
        int count = 0;
        while (offset < line_len && sscanf(line + offset, "%ld", &buff) == 1)
        {
            nums[count] = buff;
            offset += num_digits(buff) + 1;
            count += 1;
        }

        // printf("%ld: ", target);
        // for (size_t i = 0; i < count; i++)
        // {
        //     printf("%ld ", nums[i]);
        // }
        // printf("\n");

        int combinations = 1 << count;

        for (size_t c = 0; c < combinations; c++)
        {
            int cb = c;
            for (size_t i = 0; i < count - 1; i++)
            {
                if (cb & 1)
                {
                    ops[i] = '+';
                }
                else
                {
                    ops[i] = '*';
                }
                cb = cb >> 1;
            }

            if (target == eval(nums, count, ops))
            {
                part1 += target;
                break;
            }
        }

        long max_combinations = ipow(3, count - 1);
        for (size_t c = 0; c < max_combinations; c++)
        {
            size_t cb = c;
            for (size_t i = 0; i < count - 1; i++)
            {
                if (cb % 3 == 0)
                {
                    ops[i] = '+';
                }
                else if (cb % 3 == 1)
                {
                    ops[i] = '*';
                }
                else if (cb % 3 == 2)
                {
                    ops[i] = '|';
                }
                cb = cb / 3;
            }

            if (target == eval2(nums, count, ops))
            {
                // printf("Found\t%ld=", target);
                // for (size_t i = 0; i < count; i++)
                // {
                //     printf("%d", nums[i]);
                //     if (i < count - 1)
                //     {
                //         printf("%c", ops[i]);
                //     }
                // }
                // printf("\n");
                part2 += target;
                break;
            }
        }
    }

    // Close the file
    fclose(fptr);

    // part 1
    printf("\npart1: %ld\n", part1);
    printf("\npart2: %ld\n", part2);

    return 0;
}