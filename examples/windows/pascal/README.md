# Pascal example

### Embed a music (wav) inside code

This example embed the wav sound file *tPORt - Virtual DJ Studio 4.5crk.wav* inside the unit `tPORt.pas`.

The command line used to create the file:

    bin2src.exe --out-language pascal --hex --out-file tPORt "tPORt - Virtual DJ Studio 4.5crk.wav"

The new unit, is used inside [example.pas][6] and the command line to compile the example 
with [Free Pascal][1] (64bits - v3.2.0) was:

    ppcrossx64 -O3 -OpCOREAVX2 example.pas

<sub>Verfy if your system needs any other compile flags or configurations</sub>

The output binary data at `tPORt.pas` are accessed within [`example.pas`][6] through the variable 
'Tport_data' and the music is played with [Windows Multimedia API][2].

### Credits:

[Music][3] (converted from mp3 to wav) by [tPORt][4] on [Evangelion Server][5]

[1]: https://www.freepascal.org/
[2]: https://docs.microsoft.com/en-us/windows/win32/api/mmeapi/
[3]: http://pub.keygenmusic.org/music_mp3/tPORt/tPORt%20-%20Virtual%20DJ%20Studio%204.5crk.mp3
[4]: http://pub.keygenmusic.org/music_mp3/tPORt/
[5]: http://pub.keygenmusic.org/
[6]: ./example.pas


