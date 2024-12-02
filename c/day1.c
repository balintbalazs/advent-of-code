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

   int sum = 0;
   // Print vec_left
   for (size_t i = 0; i < (vec_left.size); i++)
   {
      vec_left.head[i] = abs((vec_left.head[i] - vec_right.head[i]));
      sum += vec_left.head[i];
   }

   printf("%d\n", sum);

   free_vec(vec_left);
   free_vec(vec_right);
   

   return 0;
}