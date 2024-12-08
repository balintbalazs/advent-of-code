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

bool is_valid(long *nums, int count, long target, bool part2)
{

    if (count == 1) {
        return nums[0] == target;
    }
    long c1[50];
    long c2[50];
    long c3[50];

    for (size_t i = 0; i < count; i++)
    {
        c1[i] = nums[i];
        c2[i] = nums[i];
        c3[i] = nums[i];
    }
    c1[1] = c1[1] + c1[0];
    c2[1] = c2[1] * c2[0];
    c3[1] = ipow(10, num_digits(c3[1])) * c3[0] + c3[1];
    
    if (is_valid(c1+1, count-1, target, part2)) {
        return true;
    }
    if (is_valid(c2+1, count-1, target, part2)) {
        return true;
    }
    if (part2 && is_valid(c3+1, count-1, target, part2)) {
        return true;
    }
    // printf("result: %ld\n\n", res);
    return false;
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

        if (is_valid(nums, count, target, false))
        {
            part1 += target;
        }
        if (is_valid(nums, count, target, true))
        {
            part2 += target;
        }
    }

    // Close the file
    fclose(fptr);

    // part 1
    printf("\npart1: %ld\n", part1);
    printf("\npart2: %ld\n", part2);

    return 0;
}