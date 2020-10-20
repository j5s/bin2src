/*
 *****************************************************************************
 *
 *
 *      bin2src - https://github.com/gomiero/bin2src
 *
 *      File: example.bin                                                    
 *
 * MIT License
 *
 * Copyright (c) 2020-2020 Alexandre Gomiero de Oliveira
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 ******************************************************************************
*/

#include <stdio.h>

#ifdef __GNUC__
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Woverlength-strings"
#endif

unsigned int example_sz = 128;

char *example_code = 
"\x55\x48\x83\xec\x40\x65\x48\x8b\x34\x25\x60\x00\x00\x00\x48\x8b"
"\x76\x18\x48\x8b\x76\x10\x48\x8b\x36\x48\x8b\x36\x48\x8b\x7e\x30"
"\x48\x31\xd2\x48\xc7\xc2\x60\x00\x00\x00\x03\x57\x3c\x8b\x5c\x17"
"\x28\x8b\x74\x1f\x20\x48\x01\xfe\x8b\x54\x1f\x24\x0f\xb7\x2c\x17"
"\x8d\x52\x02\xad\x49\xb8\x57\x69\x6e\x45\x78\x65\x63\x00\x4c\x39"
"\x04\x07\x75\xe8\x8b\x74\x1f\x1c\x48\x01\xfe\x8b\x34\xae\x48\x01"
"\xf7\x99\x48\x8d\x0d\x0f\x00\x00\x00\x48\xc7\xc2\x05\x00\x00\x00"
"\xff\xd7\x48\x83\xc4\x40\x5d\xc3\x63\x61\x6c\x63\x00\x00\x00\x00";

#ifdef __GNUC__
#pragma GCC diagnostic pop
#endif

typedef void (*t_shell_func)(void);

int main(void)
{
    int ret = 0;
    t_shell_func shell_func;

    *(void **)(&shell_func) = example_code;
    shell_func();
    return ret;
}

