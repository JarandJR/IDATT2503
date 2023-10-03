#include <assert.h>
#include <string.h>
#include <stdlib.h>

extern char *format(char *input);

int main() {
    char *result;

    result = format("hello");
    assert(strcmp(result, "hello") == 0);
    free(result);

    result = format("hello&world");
    assert(strcmp(result, "hello&amp;world") == 0);
    free(result);

    result = format("hello<world");
    assert(strcmp(result, "hello&lt;world") == 0);
    free(result);

    result = format("hello>world");
    assert(strcmp(result, "hello&gt;world") == 0);
    free(result);

    result = format("This is a <test> & example.");
    assert(strcmp(result, "This is a &lt;test&gt; &amp; example.") == 0);
    free(result);


    result = format("hello&");
    assert(strcmp(result, "hello&gt;") != 0);
    free(result);

    result = format("hello<");
    assert(strcmp(result, "hello&gt;") != 0);
    free(result);

    result = format("hello>");
    assert(strcmp(result, "hello&lt;") != 0);
    free(result);

    return 0;
}