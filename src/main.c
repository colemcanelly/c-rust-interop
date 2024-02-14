#include <stdio.h>
#include "lib.h"


int main () {
	puts("[    main()]\tHello from C!");
	int my_num = 2;
	times2(&my_num);
	printf("[    main()]\tHere is my number: %d\n", my_num);
    rs_func(&my_num);
	printf("[    main()]\tHere is my number now: %d\n", my_num);
	return 0;
}   