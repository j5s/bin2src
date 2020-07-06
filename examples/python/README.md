# Python example

##### Embed an image inside code

This example embed the image *simon-maage-unsplash.png* inside the module `smimgpng.py` .

The command line used to create the module:

    bin2src --out-language python --hex --out-file smimgpng simon-maage-unsplash.png


The output file `smimgpng.py` contains the binary data from the image and it is imported 
in the file [`example.py`][4], wich shows the image inside a `tkinter` frame.
    
Credits:

[Photo][1] by [Simon Maage][2] on [Unsplash][3]

[1]: https://unsplash.com/photos/C9dhUVP-o6w
[2]: https://unsplash.com/@simonmaage?utm_source=unsplash&amp;utm_medium=referral&amp;utm_content=creditCopyText
[3]: https://unsplash.com/images/things/airplane?utm_source=unsplash&amp;utm_medium=referral&amp;utm_content=creditCopyText
[4]: ./example.py
