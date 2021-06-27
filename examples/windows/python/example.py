#
# Embed a photo data inside a Tk frame
#
import tkinter as tk
import smimgpng as smimg

AUTHOR = "Alexandre Gomiero de Oliveira"
REPO = "https://github.com/gomiero/bin2src"

class App(tk.Frame):
    def __init__(self, master):
        super().__init__(master)
        self.config(width=427, height=640)
        canvas = tk.Canvas(self, width=427, height=640, bg="black")
        canvas.pack()
        # --> Read image from binary data generated at smimgpng.py <--
        self.photo_img = tk.PhotoImage(format = 'png', data = smimg.SMIMGPNG_DATA)
        canvas.create_image(0, 0, image = self.photo_img, anchor=tk.NW)
        self.pack()

# Entry point: create the root window...
root = tk.Tk()
# ...the App instance...
app = App(master = root)
# ...and run the main loop.
app.mainloop()

