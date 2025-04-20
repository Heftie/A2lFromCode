// a2l on
// a2l-unit °deg
uint32_t arraytest[32] = { 0, 1, 2, 3};

/*
a2l on
a2l-unit m/s
*/
float velo = 4.0;

// a2l on
/*
a2l-unit mm
*/
#ifdef Test
uint16_t var = 5;
#else
int var = 10;
#endif

int varint = 0;

void func();

void func1(uint32_t a, uint32_t b)
{
    uint32_t c = a + b;
    if (c > 0)
    {
        c = 0;
    }
    else
    {
        c = 1;
    }
}