#include <assert.h>
#include <string.h>
#include <stdlib.h>

extern char *format_string(char *in);

int main() {
    char *result;

    result = format_string("hello");
    assert(strcmp(result, "hello") == 0);
    free(result);

    result = format_string("hello&world");
    assert(strcmp(result, "hello&amp;world") == 0);
    free(result);

    result = format_string("hello<world");
    assert(strcmp(result, "hello&lt;world") == 0);
    free(result);

    result = format_string("hello>world");
    assert(strcmp(result, "hello&gt;world") == 0);
    free(result);

    result = format_string("This is a <test> & example.");
    assert(strcmp(result, "This is a &lt;test&gt; &amp; example.") == 0);
    free(result);


    result = format_string("hello&");
    assert(strcmp(result, "hello&gt;") != 0);
    free(result);

    result = format_string("hello<");
    assert(strcmp(result, "hello&gt;") != 0);
    free(result);

    result = format_string("hello>");
    assert(strcmp(result, "hello&lt;") != 0);
    free(result);

    return 0;
}