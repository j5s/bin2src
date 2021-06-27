# bin2src

> **bin2src** is a simple command line that converts a binary file to an array of bytes, 
assigned to a variable in a source file of another language; therefore you can embed 
it into your program.

Currently supported output languages:

* C
* C (shellcode)
* Pascal
* Python
* Rust


<a name="overview"></a>
## Overview

**bin2src** reads a binary file (e.g. jpg, wav, mp3, etc.) and generates a source file 
with the binary data embeded into it as a byte array assigned to a variable.  

It is is useful if you don't want to distribute the binary file with your application and 
let users have directly access to it.  

With **bin2src** you can embed the data inside the final executable and use it through the
variable stored in memory.   

Keep in mind that it is always possible for an "advanced" user, to extract the data, either
from the executable or a memory dump.

<a name="usage"></a>
## Usage

<pre>
bin2src < -l LANG | --out-lang LANG > [ OPTIONS ] < FILE >

LANG and FILE are required.

Options:

        -l, --out-language LANG         specify the language, where LANG={c|cshell|pascal|python|rust}

        -d, --out-dir PATH              specify where to output source(s) file(s);
                                        if not specified, generate in current directory

        -f, --out-file OUTFILE          specify the output file(s) name (* without extension *);
                                        if not specified, output file(s) will have the same name
                                        of input file (without extra dots).

        -h, --hex                       output bytes in hexadecimal (for C shellcode this flag has
                                        diferent behaviors. See the Github site for more information)

Currently supported languages:

  - C
  - C for shellcode
  - Pascal
  - Python
  - Rust	
</pre>

## Examples

Suppose you have an image `myimage.jpg` that you want to embeed into your executable:
<br>
<br>

<a name="example1"></a>
**Example 1:**

```
bin2src --out-language pascal --out-dir "X:\My Projects\project01" --out-file image01 myimage.jpg
```

<sub>Windows paths with spaces needs quotation marks</sub>

This command will create the file `...\image01.pas` with the data defined using decimal 
format: `[210, 0, ...]`.
<br>
<br>

<a name="example2"></a>
**Example 2:**

```
bin2src -l c -d "X:\My Projects\project02" -f image01 -h myimage.jpg
```

This command will create the files, but with data in hexadecimal format:
`[0x10, 0xfa, ...]`:

* `...\image01.h`
* `...\image01.c`

<br>

<a name="example3"></a>
**Example 3:**

```
bin2src --out-language python myimage.jpg
```

This command will create the file `myimage.py` at the current directory.
<br>
<br>
Check other examples at [examples directory][3] for some practical uses of bin2src.

## Atention

* Beware of the **file size** that you will embed in your code!!!

  Ensure that it is accepted by your O.S., compiler, language standards, memory at 
  runtime, etc.

* If the input file name has more dots, in addition to the dot that splits the name and
  the extension, without the `--out-file` or `-f` command line option the output 
  file name will be the first name before the first dot. Example:
  
  `abc.def.ghi.x` => `abc.y`
  
* The option `--hex` or `-h` for C shellcode output works differently. Without this flag, 
  it will define the data type as `unsigned char` bytes, otherwise with the hexadecimal
  flag, it will embed the bytes as a string type (`char *`).
  
* To embed the generated C shellcode using string format, make sure that the binary data 
  doesn't contains null bytes ("\x00") or don't use string functions from stdlib, like 
  `strlen`. This may break your code and could raise exceptions (e.g. access violations, 
  etc.).

* All of the tests were made with Windows 10 Pro (2004), and to execute the alpha release
  maybe you have to install the latest [MSVC runtime][4]. Even though, it should works 
  well on other platforms.
  
* There are a lot of things to organize and improvements to do. Suggestions are always 
  welcome.

<a name="license"></a>
## License

Developed by Alexandre Gomiero de Oliveira under the [MIT License][1].

Any code generated by **bin2src** are under [MIT License][2].

Please, get in contact with me if you need other license type.

[1]: ./LICENSE
[2]: ./LICENSE-GENERATED
[3]: ./examples
[4]: https://support.microsoft.com/en-us/help/2977003/the-latest-supported-visual-c-downloads
[5]: ./TODO.md
