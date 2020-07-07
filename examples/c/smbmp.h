/*
 *****************************************************************************
 *
 *
 *      bin2src - https://github.com/gomiero/bin2src
 *
 *      File: simon-maage-unsplash.bmp                                       
 *
 ******************************************************************************
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

