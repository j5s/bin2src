# Rust example

### Embed an archive inside code

This example shows how to embed a archive inside a program and decompress it on-the-fly.

As a proof of concept, the decompressed data is written back to a file for 
future comparison.

---

The text file `lorem.txt` is a "[*lorem ipsum*][1]", compacted with [bzip2][2]:

    bzip2 -9 lorem.txt
    
The output file `lorem.txt.bz2` is converted by **bin2src** to a Rust module.

The command line to create the module:

    bin2src --out-language rust --hex --out-dir src lorem.txt.bz2

The `out-dir` parameter was used to generate the module inside `./src` dir.

The output file `lorem.rs` contains the binary data from the archive lorem.txt.bz2, 
wich is imported inside the file [main.rs][9].

To run the program, simply run:

    cargo run
    
It will create an output file `lorem.out.txt`, with the same content of the initial file.

The bzip2 lib was linked statically within the executable (see [build.rs][10]), and 
two libraries (MSVC compatible - see Credits) were used for tests.

### Credits:

[philr/bzip2-windows][3] by [Phil Ross][4]

[ShiftMediaProject/bzip2][5] by [ShiftMediaProject][6]

[bzip2 algorithm][2] by Julian Seward (see [LICENSE][8])

[Wikipedia][7] for the *lorem ipsum* definition.

[1]: https://en.wikipedia.org/wiki/Lorem_ipsum
[2]: https://www.sourceware.org/bzip2/
[3]: https://github.com/philr/bzip2-windows
[4]: https://github.com/philr
[5]: https://github.com/ShiftMediaProject/bzip2
[6]: https://github.com/ShiftMediaProject
[7]: https://en.wikipedia.org/wiki/Main_Page
[8]: ./LICENSE
[9]: ./src/main.rs
[10]: ./build.rs
