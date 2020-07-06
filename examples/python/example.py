#
# Embed photo data inside a Tk frame
#
import tkinter as tk
import smimgpng as smimg

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

root = tk.Tk()
app = App(master = root)
app.mainloop()

