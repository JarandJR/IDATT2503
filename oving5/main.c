#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "format_string.h"

int main() {
    char in [30];
    strcpy(in, "This is a <test> & example.");
    printf("%s\n", format_string(in));
    return 0;
}