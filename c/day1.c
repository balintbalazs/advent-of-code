#include <stdio.h>
#include "vec.h"

int main()
{
   // printf() displays the string inside quotation
   printf("Hello, World!\n");

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
      printf("left: %d, right: %d\n", left, right);
      push_vec(&vec_left, left);
      push_vec(&vec_right, right);
   }

   // Close the file
   fclose(fptr);

   // Print vec_left
   for (size_t i = 0; i < (vec_left.size); i++)
   {
      printf("left: %d\n", vec_left.head[i]);
   }

   free_vec(vec_left);
   free_vec(vec_right);
   

   return 0;
}