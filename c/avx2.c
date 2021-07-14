#include <stdio.h>
#include <immintrin.h>

typedef unsigned char ubyte;
typedef ubyte ubyte32[32] __attribute__((aligned(32)));

void add(ubyte* dest, ubyte* a, ubyte* b)
{
    __m256i va = _mm256_load_si256((__m256i*)a);
    __m256i vb = _mm256_load_si256((__m256i*)b);
    __m256i added = _mm256_add_epi8(va, vb);
    _mm256_store_si256((__m256i*)dest, added);
}

void sub(ubyte* dest, ubyte* a, ubyte* b)
{
    __m256i va = _mm256_load_si256((__m256i*)a);
    __m256i vb = _mm256_load_si256((__m256i*)b);
    __m256i added = _mm256_sub_epi8(va, vb);
    _mm256_store_si256((__m256i*)dest, added);
}


int main()
{
    ubyte32 res;
    ubyte32 a = {250, 251, 252, 253, 254, 255, 255, 255, 255, 255, 255, 240};
    ubyte32 b = {1, 1, 1, 1, 1, 1, 2, 3, 4, 5,
                 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
                 31, 31
                };
    add(res, a, b);

    for (int i = 0; i < sizeof(res); i++) {
        printf("%d\n", res[i]);
    }
}
