# Rust example

### Embed an archive inside code

This example shows how to embed a archive inside the program and decompress it on-the-fly.

As a proof of concept, the decompressed data is wrote to a file for comparison.

---

The text file `lorem.txt` is a "[*lorem ipsum*][1]", compacted with [bzip2][2]:

    bzip2 -9 lorem.txt
    
The output file `lorem.txt.bz2` is submited to **bin2src** to create a Rust module.

The command line used to create the module:

    bin2src --out-language rust --hex --out-dir src lorem.txt.bz2

The `out-dir` parameter was used to generate the module inside `./src` dir.

The output file `lorem.rs` contains the binary data from the archive lorem.txt.bz2, 
and it is imported inside the file [main.rs][9].

To run the program, simply run the command:

    cargo run
    
It'll create an output file `lorem.out.txt`, and this file has the exact same 
content of the initial file.

The bzip2 lib was linked statically within the executable (see [build.rs][10]), and 
two binary libraries (MSVC compatible - see Credits) was used for tests.

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
