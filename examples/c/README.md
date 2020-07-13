# C example

### Embed an image inside code

This example embed the image *simon-maage-unsplash.bmp* inside the files:

- [smbmp.h][6] (header)
- smbmp.c (data)

The command line used to create the files:

    bin2src.exe -l c -h -f smbmp simon-maage-unsplash.bmp

The output binary data at `smbmp.c` are accessed within [`example.c`][5] by the variable 
'smbmp_data' to create a [device independent bitmap][4]  (DIB), which is displayed inside 
the client area of a windows.

---

The test [`example.c`][5] can be compiled (without any changes) with:

- Visual Studio Community 2019 (MSVC - Version 19.26.28806 - x64)<sup>*</sup> 
- MinGW64 gcc version 10.1.0 (Rev3, Built by MSYS2 project)<sup>*</sup> 


For MSVC, just create a C++ project, import all source files (.c and .h) and run the project.

For MinGW64, run the command bellow to compile and create the executable:

    gcc -Wall -Wextra -pedantic -O3 -mwindows -m64 -o example.exe example.c smbmp.c


<sub>* these versions was used for the tests</sub>

### Credits:

[Photo][1] by [Simon Maage][2] on [Unsplash][3]

[1]: https://unsplash.com/photos/C9dhUVP-o6w
[2]: https://unsplash.com/@simonmaage?utm_source=unsplash&amp;utm_medium=referral&amp;utm_content=creditCopyText
[3]: https://unsplash.com/images/things/airplane?utm_source=unsplash&amp;utm_medium=referral&amp;utm_content=creditCopyText
[4]: https://docs.microsoft.com/en-us/windows/win32/gdi/device-independent-bitmaps
[5]: ./example.c
[6]: ./smbmp.h