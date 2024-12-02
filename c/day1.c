#include <stdio.h>
#include "vec.h"

int main()
{
   FILE *fptr;

   // Open a file in read mode
   fptr = fopen("../inputs/day1.in", "r");

   // Early exit if the file does not exist
   if (fptr == NULL)
   {
      perror("Not able to open the file.\n");
      return 1;
   }

   // 2 vectors for each column
   Vec vec_left = create_vec(10);
   Vec vec_right = create_vec(10);

   int left;
   int right;

   // Read the content and print it
   while (fscanf(fptr, "%d %d", &left, &right) == 2)
   {
      // printf("left: %d, right: %d\n", left, right);
      push_vec(&vec_left, left);
      push_vec(&vec_right, right);
   }
   // Close the file
   fclose(fptr);

   sort_vec(&vec_left);
   sort_vec(&vec_right);

   int part1 = 0;
   // Print vec_left
   for (size_t i = 0; i < (vec_left.size); i++)
   {
      int diff = abs((vec_left.head[i] - vec_right.head[i]));
      part1 += diff;
   }

   printf("%d\n", part1);

   int max_right = vec_right.head[vec_right.size - 1];
   
   Vec counts_right = create_vec(max_right);

   // Count occurances in right vector
   for (size_t i = 0; i < (vec_right.size); i++)
   {
      counts_right.head[vec_right.head[i]] += 1; 
   }

   int part2 = 0;
   // Check simliarity with left vector
   for (size_t i = 0; i < (vec_left.size); i++)
   {
      int similarity = counts_right.head[vec_left.head[i]] * vec_left.head[i]; 
      part2 += similarity;
   }

    printf("%d\n", part2);

   free_vec(vec_left);
   free_vec(vec_right);
   free_vec(counts_right);

   return 0;
}