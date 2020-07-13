/*
 *******************************************************************************
 *
 *
 *      bin2src - https://github.com/gomiero/bin2src
 *
 *      File: simon-maage-unsplash.bmp                                       
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
 *******************************************************************************
*/

#ifndef __SMBMP__HEADER
#define __SMBMP__HEADER

// Very simple iterator implementation
void restart();
unsigned char* begin();
unsigned char* next();
unsigned char* end();
int set_pos(unsigned int p);
unsigned char* get_pos(unsigned int p);

extern unsigned int smbmp_sz;
extern unsigned char smbmp_data[];

#endif

