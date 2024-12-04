#include <stdio.h>
#include <string.h>
#include <ctype.h>

#define bool int
#define true 1
#define false 0

int num_digits(int num)
{

    int digits = 0;
    do
    {
        num = num / 10;
        digits += 1;
    } while (num);
    return digits;
}

int main()
{
    FILE *fptr;

    // Open a file in read mode
    fptr = fopen("../inputs/day3.in", "r");

    // Early exit if the file does not exist
    if (fptr == NULL)
    {
        perror("Not able to open the file.\n");
        return 1;
    }

    int lhs;
    int rhs;

    char *line = NULL;
    size_t line_bufer_len = 0;
    size_t line_len = 0;

    int part1 = 0;
    int part2 = 0;

    bool enabled = true;

    while (getline(&line, &line_bufer_len, fptr) != -1)
    {
        char *pos = line;

        // part 1
        while (true)
        {
            pos = strstr(pos, "mul(");
            if (!pos)
                break;

            pos += 4;
            if (!isdigit(*pos))
                continue;

            lhs = atoi(pos);
            pos += num_digits(lhs);
            if (*pos != ',')
                continue;

            pos += 1;
            if (!isdigit(*pos))
                continue;

            rhs = atoi(pos);
            pos += num_digits(rhs);
            if (*pos != ')')
                continue;

            part1 += lhs * rhs;
        }

        pos = line;

        // part 2
        while (true)
        {
            char *mul_pos = strstr(pos, "mul(");
            if (!mul_pos)
                break;

            char *do_pos = strstr(pos, "do()");
            char *dont_pos = strstr(pos, "don't()");
            if (do_pos && !dont_pos)
            {
                if (do_pos < mul_pos)
                {
                    enabled = true;
                    pos = do_pos + 4;
                    continue;
                }
                else
                {
                    pos = mul_pos + 4;
                }
            }
            else if (dont_pos && !do_pos)
            {
                if (dont_pos < mul_pos)
                {
                    enabled = false;
                    pos = dont_pos + 7;
                    continue;
                }
                else
                {
                    pos = mul_pos + 4;
                }
            }
            else if (do_pos && dont_pos)
            {
                if (do_pos < dont_pos)
                {
                    if (do_pos < mul_pos)
                    {
                        enabled = true;
                        pos = do_pos + 4;
                        continue;
                    }
                    else
                    {
                        pos = mul_pos + 4;
                    }
                }
                else
                {
                    if (dont_pos < mul_pos)
                    {
                        enabled = false;
                        pos = dont_pos + 7;
                        continue;
                    }
                    else
                    {
                        pos = mul_pos + 4;
                    }
                }
            }

            // processing mul
            pos = mul_pos + 4;

            if (!isdigit(*pos))
                continue;

            lhs = atoi(pos);
            pos += num_digits(lhs);
            if (*pos != ',')
                continue;

            pos += 1;
            if (!isdigit(*pos))
                continue;

            rhs = atoi(pos);
            pos += num_digits(rhs);
            if (*pos != ')')
                continue;
            printf("mul(%d,%d)\n", lhs, rhs);
            if (enabled)
                part2 += lhs * rhs;
        }
    }

    printf("%d\n", part1);
    printf("%d\n", part2);

    fclose(fptr);

    return 0;
}