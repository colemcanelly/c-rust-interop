#include <stdio.h>
#include <string.h>
#include "lib.h"

const char* c_func (int num) {
	printf("[  c_func()]\tHere is your number: %d\n", num);
	return "{ response from c_func() }";
}

void times2 (int* n) {
	puts("[  times2()]\tDoubling...");
	*n *= 2;
}