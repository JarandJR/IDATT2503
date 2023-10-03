#include <stdio.h>
#include <string.h>
#include "format_string.h"

int main() {
    char in [30];
    strcpy(in, "This is a <test> & example.");
    printf("%s\n", format(in));
    return 0;
}