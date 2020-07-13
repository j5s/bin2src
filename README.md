# bin2src

> **bin2src** is a simple command line that converts a binary file to an array of bytes, defined at a source of another language, so you can embed it into your program.

Currently supported output languages:

* C
* C (shellcode)
* Pascal
* Python
* Rust


<a name="overview"></a>
## Overview

**bin2src** reads a binary file (.jpg, .wav, .mp3, etc.) and generate a source file with the binary
data embeded into it as a byte array.  

Sometimes, maybe you don't want to distribute a binary file inside your program's package, so 
the user can't access it directly.  

With **bin2src** you can embed it inside the executable and read the bytes direct from memory.   

Keep in mind that it's always possible for an "advanced" user extract the file, even inside the 
executable.

### Give a Star! :star:
If you like this project and find it useful, please give it a star. I appreciate very much! 
Thanks!

<a name="usage"></a>
## Usage

<pre>
bin2src < -l LANGUAGE | --out-lang LANGUAGE > [OPTIONS] < FILE >


LANGUAGE and FILE are required!

Options:

	-l, --out-language LANGUAGE     specify the language, where LANGUAGE={c|cshell|pascal|python|rust}
	-d, --out-dir PATH              specify where to output source(s) file(s) if not specified,
	                                generate in current directory
	-f, --out-file OUTFILE          specify the output file(s) name (* without extension! *). If not
	                                specified, output file(s) will have the same name of input file
	-h, --hex                       output bytes in hexadecimal (default for shellcode)
	
</pre>

## Examples

Suppose you have an image `myimage.jpg`:
<br>
<br>

<a name="example1"></a>
**Example 1:**

```
bin2src --out-language pascal --out-dir "X:\My Projects\project01" --out-file image01 myimage.jpg
```

will create the file `X:\My Projects\project01\image01.pas` with bytes in decimal format: `[210, 0, ...]`.
<br>
<br>

<a name="example2"></a>
**Example 2:**

```
bin2src -l c -d "X:\My Projects\project02" -f image01 -h myimage.jpg
```

will create the files (with bytes in hexadecimal: `[0x10, 0xfa, ...]`):

* `X:\My Projects\project01\image01.h`
* `X:\My Projects\project01\image01.c`

<br>

<a name="example3"></a>
**Example 3:**

```
bin2src --out-language python myimage.jpg
```

will create the file "myimage.py" at the current directory.
<br>
<br>

## Atention

* Beware with the **file size** that you'll embed in your code!!!

  Verify if it's accepted by your O.S., compiler, language standards, memory at runtime, etc.

* if the file has more dots, in addition to the dot that separates the extension name and
  you don't use the `--out-file` or `-f` command line option, the output file name will 
  be the first name before the first dot. Example (generating a 'y' file):
  
  `abc.def.ghi.x` => `abc.y`

<a name="license"></a>
## License

Developed by Alexandre Gomiero de Oliveira under the [GPL-3.0 License][1].

Any code generated by **bin2src** is under [MIT License][2]

Please contact me if you'll use it for commercial projects (ex: use the tool to generate code for
closed source projects that you'll sell, patent, etc.).

I'm open to make an agreement about the commercial use of the tool (even to change the project 
to a more 'open' license), or maybe you can consider make a donation to help me with future 
projects. :smiley: :thumbsup:


[1]: ./LICENSE
[2]: ./LICENSE-GENERATED
