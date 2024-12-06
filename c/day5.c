#include <stdio.h>
#include "vec.h"

#define bool int
#define true 1
#define false 0

int main()
{
   FILE *fptr;

   // Open a file in read mode
   fptr = fopen("../inputs/day5.in", "r");

   // Early exit if the file does not exist
   if (fptr == NULL)
   {
      perror("Not able to open the file.\n");
      return 1;
   }

   // 2 vectors for each column
   Vec ordering = create_vec(10);

   int left;
   int right;

   // Read ordering rules
   long pos;
   while (fscanf(fptr, "%d|%d\n", &left, &right) == 2)
   {
      // save pos after successful fscan
      // fscan pattern will consume all whitespace chars after each line
      // at the last successful fscan, ftell returns the pos of the beginning
      // of the second input section
      pos = ftell(fptr);
      // all numbers are 2 digits
      push_vec(&ordering, left * 100 + right);
   }

   // last fscan that fails will consume first number of updates section
   int ret = fseek(fptr, pos, SEEK_SET);
   if (ret != 0)
   {
      perror("Oh no..");
      return 1;
   }

   // read updates
   char *line = NULL;
   size_t line_bufer_len = 0;
   size_t line_len = 0;

   // probably 1000 lines are enough
   Vec updates[1000];

   int line_index = 0;
   while (getline(&line, &line_bufer_len, fptr) != -1)
   {
      line_len = strlen(line);

      Vec page_numbers = create_vec(100);
      char *end = line;
      size_t page_index = 0;
      while (end < line + line_len - 1)
      {
         push_vec(&page_numbers, strtol(end, &end, 10));
         page_index += 1;
         // skip ,
         end += 1;
      }

      updates[line_index] = page_numbers;
      line_index += 1;
   }

   // somehow off by 1? an extra line is read
   int line_count = line_index;

   // Close the file
   fclose(fptr);

   // part 1

   // sort ordering rules so we can do binary search later
   sort_vec(&ordering);

   int part1 = 0;
   int part2 = 0;

   for (size_t l = 0; l < line_count; l++)
   {
      // do the stuff
      bool valid = true;
      for (size_t i = 0; i < updates[l].size - 1; i++)
      {
         for (size_t j = i + 1; j < updates[l].size; j++)
         {
            int invalid_rule = updates[l].head[i] + 100 * updates[l].head[j];

            if (sorted_vec_contains(&ordering, invalid_rule))
            {
               valid = false;
               // swap invalid  values
               int tmp = updates[l].head[i];
               updates[l].head[i] = updates[l].head[j];
               updates[l].head[j] = tmp;
               // start from beginning
               i = 0;
               j = 1;
            }
         }
         // only used in part 1
         // if (!valid) {
         //    continue;
         // }
      }

      if (valid)
      {
         size_t middle = updates[l].size / 2;
         part1 += updates[l].head[middle];
      }
      else
      {
         size_t middle = updates[l].size / 2;
         part2 += updates[l].head[middle];
      }
   }

   printf("\npart1: %d\n", part1);
   printf("\npart2: %d\n", part2);

   // clean up
   for (size_t line = 0; line < line_count; line++)
   {
      free_vec(updates[line]);
   }

   free_vec(ordering);

   return 0;
}