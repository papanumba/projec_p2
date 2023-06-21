# figures.py

import tkinter as tk


# Pt, Ln, Eq, Cn share
class Figure:
    def __init__(self, root, idf, del_cbk, fig_text):
        # root: tk.Frame     # root frame to which append the Figure widgets
        # idf: int            # used when calling del_cbk
        # del_cbk: void fn() # delete_callback, for the destroy button X
        # fig_text: str      # text to be shown on the label ("eq", etc.)
        self.idf = idf
        f = tk.Frame(root)
        f.pack()
        for j in range(3):
            f.columnconfigure(j, weight=1)
        self.frame = f
        self.label = tk.Label(self.frame, text=fig_text)
        self.label.grid(row=0, column=0)
        self.x_btn = tk.Button(
            self.frame,
            text="X",
            fg="#FFFFFF",
            bg="#DB0000",
            activeforeground="#000000",
            activebackground="#FF2400",
            command=del_cbk
        )
        self.x_btn.grid(row=0, column=2) #, expand=False)

    def get_as_str(self):
        raise NotImplementedError


def zero_if_empty(t):
    if t == "":
        t = "0.0"
    return t

def format_float(f):
    f = zero_if_empty(f)
    try:
        f = float(f)
        f = "{:.4f}".format(f)
    except:
        pass
    return f


class Eq(Figure):
    def __init__(self, root, idf, del_cbk):
        super().__init__(root, idf, del_cbk, "eq")
        self.vec = []
        for j in range(3):
            e = tk.Entry(self.frame)
            e.grid(row=1, column=j)
            self.vec.append(e)

    def get_as_str(self):
        s = ""
        for j in range(3):
            s += format_float(self.vec[j].get()) + " "
        s = s[:-1]
        return s


class Cn(Figure):
    def __init__(self, root, idf, del_cbk):
        super().__init__(root, idf, del_cbk, "cn")
        self.mat = []
        for i in range(3):
            row = []
            for j in range(3):
                e = tk.Entry(self.frame)
                e.grid(row=i+1, column=j)
                row.append(e)
            self.mat.append(row)

    def get_as_str(self):
        s = ""
        for i in range(3):
            for j in range(3):
                s += format_float(self.mat[i][j].get()) + " "
            s = s[:-1] + "\n"
        s = s[:-1]
        return s

class Param:
    def __init__(self, rootframe, name, cbk):
        self.name = name
        self.frame = tk.Frame(rootframe)
        self.frame.pack(anchor=tk.S)
        self.label = tk.Label(frame, name)
        self.label.grid(row=0, column=0)
        self.scale = tk.Scale(
            self.frame,
            from_=-1.0,
            to=1.0,
            orient=tk.HORIZONTAL,
            command=cbk
        )
        self.scale.grid(row=1, column=0)

    def get(self):
        return self.scale.get()
