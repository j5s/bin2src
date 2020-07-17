# C shellcode example

### Runs the "open calc" example

This example is a common C shellcode that opens the calculator on windows.

There are two version of the same example, one with Intel syntax and other with 
AT&T syntax.

<a name="GAS"></a>
### AT&T

The assembly source code [example.S][3] was compiled with GNU assembler and the 
binary `.text` section of the object file was extracted with objcopy, 
with the commands:

    as --64 example.S -o example.o
    
    objcopy -O binary -j .text example.o example.bin


The command line used to create the template C file with the shellcode:

    bin2src --out-language cshell --hex example.bin

And the binary was converted to a C template file [example.c][4].

The template was duplicated to [example_final.c][5] and altered to include the `windows.h` header, 
alloc a virtual page with read, write and execute access where the shellcode was transfered.

The variable `shell_func` was assigned to the new page allocated (at the template,
it was a direct pointer to the shellcode and that doesn't works because of the 
memory protection at data pages) and executed as a function.

After the execution, the Windows Calculator is at the desktop and the page is released.

Versions used for tests:

 * MinGW GNU assembler (GNU Binutils) 2.34
 * MinGW GNU objcopy (GNU Binutils) 2.34
    
### NASM

The assembly source [example.asm][6] was compiled to a binary file with the command:

    nasm -f bin -o example.bin example.asm

The command used to generate the template C file was:

    bin2src -l cshell example.bin
    
The output template [example.c][7] was altered the same way as the GAS to [example_final.c][8].

Version used for tests:

 * NASM version 2.15.02 compiled on Jul  1 2020
 
---

The test `example_final.c` (even though they are different for GAS and NASM) can be compiled 
without any changes with:

- Visual Studio Community 2019 (MSVC - Version 19.26.28806 - x64)<sup>*</sup> 
- MinGW64 gcc version 10.1.0 (Rev3, Built by MSYS2 project)<sup>*</sup> 

For MSVC, just create a C++ project, import the source file and run the project.

For MinGW64, run the command bellow to compile and create the executable:

    gcc -Wall -Wextra -pedantic -O3 -o example_final.exe example_final.c


<sub>* these versions was used for the tests</sub>

### Credits:

[Peter Ferrie][1] for the [win-exec-calc-shellcode][2] example on how to create a shellcode 
for Windows 64 bits.

[1]: https://github.com/peterferrie
[2]: https://github.com/peterferrie/win-exec-calc-shellcode
[3]: ./gas/example.S
[4]: ./gas/example.c
[5]: ./gas/example_final.c
[6]: ./nasm/example.asm
[7]: ./nasm/example.c
[8]: ./nasm/example_final.c 


