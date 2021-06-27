# C shellcode example

### Runs the "open calc" example

This example is a so common C shellcode that opens the windows calc.

There are two version of the same example, one using Intel syntax and the other using 
AT&T syntax.

<a name="GAS"></a>
### AT&T

The assembly source code [example.S][3] was compiled with GNU assembler and the 
binary `.text` section of the object file was extracted with objcopy:

    as --64 example.S -o example.o
    
    objcopy -O binary -j .text example.o example.bin

The command line used to create the C file with the shellcode:

    bin2src --out-language cshell --hex example.bin

The template was duplicated to [example_final.c][5] and altered to include the 
`windows.h` header, to allow the allocation of a virtual page with read, write 
and execute access where the shellcode was stored.

The variable `shell_func` was assigned to the new page (at the generated template,
it is a direct pointer to the shellcode and **it doesn't works**, because of the 
page memory protection) and executed as a function.

After the execution, the Windows Calculator will open and the allocated page will be
released.

Versions used for tests:

 * MinGW GNU assembler (GNU Binutils) 2.34
 * MinGW GNU objcopy (GNU Binutils) 2.34
    
### NASM

The assembly source [example.asm][6] was compiled and converted with the commands:

    nasm -f bin -o example.bin example.asm

    bin2src -l cshell example.bin
    
The output template [example.c][7] was altered the same way as the GAS example
[example_final.c][8].

Version used during the tests:

 * NASM version 2.15.02 compiled on Jul  1 2020
 
---

The test `example_final.c` (even though they are different for GAS and NASM) can be 
compiled without any changes with:

- Visual Studio Community 2019 (MSVC - Version 19.26.28806 - x64)<sup>*</sup> 
- MinGW64 gcc version 10.1.0 (Rev3, Built by MSYS2 project)<sup>*</sup> 

For MSVC, just create a C++ project, import the sources and run the project.

For MinGW64, run the following command to create the executable:

    gcc -Wall -Wextra -pedantic -O3 -o example_final.exe example_final.c


<sub>* these are the versions used during the tests</sub>

### Credits:

[Peter Ferrie][1] for the [win-exec-calc-shellcode][2] example on how to create a shellcode 
for Windows 64 bits.

[1]: https://github.com/peterferrie
[2]: https://github.com/peterferrie/win-exec-calc-shellcode
[3]: ./gas/example.S
[5]: ./gas/example_final.c
[6]: ./nasm/example.asm
[7]: ./nasm/example.c
[8]: ./nasm/example_final.c 


