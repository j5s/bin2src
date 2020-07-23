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

#include <windows.h>
#include <stdio.h>

// Shell code size
unsigned int example_sz = 127;

// Shell code data
unsigned char example_code[] = {
 85,  72, 131, 236,  64, 101,  72, 139,  52,  37,  96,   0,   0,   0,  72, 
139, 118,  24,  72, 139, 118,  16,  72, 139,  54,  72, 139,  54,  72, 139, 
126,  48,  72,  49, 210, 186,  96,   0,   0,   0,   3,  87,  60, 139, 156, 
 23,  40,   0,   0,   0, 139, 116,  31,  32,  72,   1, 254, 139,  84,  31, 
 36,  15, 183,  44,  23, 141,  82,   2, 173,  73, 184,  87, 105, 110,  69, 
120, 101,  99,   0,  76,  57,   4,   7, 117, 232, 139, 116,  31,  28,  72, 
  1, 254, 139,  52, 174,  72,   1, 247, 153,  72, 141,  13,  13,   0,   0, 
  0, 186,   5,   0,   0,   0, 255, 215,  72, 131, 196,  64,  93, 195,  99, 
 97, 108,  99,   0,   0,   0,   0 };

// Function defined to call shell code
typedef void (*t_shell_func)(void);

int main(void)
{
    int ret = 0;
    t_shell_func shell_func;
    LPVOID page;


    // Alloc a virtual page with read, write and execute access
    page = VirtualAlloc(NULL, 4096, MEM_COMMIT, PAGE_EXECUTE_READWRITE);

    // Copy the shellcode to the new page
    CopyMemory(page, example_code, example_sz);

    // Assign a variable to the new page
    *(void **) (&shell_func) = page;

    // Execute the code
    shell_func();

    // Release the page
    VirtualFree(page, 0, MEM_RELEASE);
    return ret;
}

