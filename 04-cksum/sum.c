#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

uint32_t sum_init(void)
{
    return 0;
}

uint32_t sum_update(uint32_t sum, uint8_t* data, size_t len)
{
    size_t i = 0;
    uint32_t s = sum;

    for (i = 0; i < len; i++) {
        uint8_t c = data[i];
        s = (s >> 1) | ((s & 1) << 15);
        s += c;
        s &= 0xffff;
    }

    return s;
}

uint32_t sum_finish(uint32_t sum)
{
    return sum;
}

uint32_t sum_file(FILE* fp)
{
    uint32_t s = sum_init();
    uint8_t buf[1024];
    size_t len = 0;

    while ((len = fread(buf, 1, sizeof(buf), fp)) > 0) {
        s = sum_update(s, buf, len);
    }

    return sum_finish(s);
}

int main(int argc, char* argv[])
{
    FILE* in = stdin;
    uint32_t sum = 0;

    if (argc > 1) {
        FILE* fd = fopen(argv[1], "rb");
        if (fd == NULL) {
            fprintf(stderr, "error: cannot open file '%s'\n", argv[1]);
            return 1;
        }

        in = fd;
    }

    sum = sum_file(in);
    printf("sum: %u 0x%08x\n", sum, sum);

    return 0;
}
