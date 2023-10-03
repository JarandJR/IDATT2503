#include <stdint.h>
#include <string.h>
#include <stdlib.h>

extern char *format_string(char *in);

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    char *str = (char *)malloc(sizeof(char) * size + 1);
    memcpy(str, data, size);
    str[size] = '\0';

    char *result = format_string(str);
    if (result != NULL) {
        free(result);
    }

    free(str);

    return 0;
}