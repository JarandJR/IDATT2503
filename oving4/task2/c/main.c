#include <stdio.h>
#include <string.h>
#include <stdlib.h>

char* format(char *in) {
    size_t l = strlen(in);
    size_t len = l;
    for (size_t i = 0; i < l; ++i) {
        if (in[i] == '&') len += 4;
        else if (in[i] == '<' || in[i] == '>') len += 3;
    }
    char *out = malloc(len + 1);
    if (out == NULL) return NULL;

    size_t pos = 0;
    for (size_t i = 0; i < strlen(in); i++) {
        if (in[i] == '&') {
            strcpy(out + pos, "&amp;");
            pos += 5;
        } else if (in[i] == '<') {
            strcpy(out + pos, "&lt;");
            pos += 4;
        } else if(in[i] == '>') {
            strcpy(out + pos, "&gt;");
            pos += 4;
        } else {
            out[pos++]=in[i];
        }
    }

    out[pos] = '\0';
    return out;
}

int main() {
    char in [30];
    strcpy(in, "This is a <test> & example.");
    printf("%s\n", format(in));
    return 0;
}