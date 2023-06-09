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
        self.text = fig_text
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
    '''try:
        f = float(f)
        f = "{:.4f}".format(f)
    except:
        pass
    '''
    return f


class Pt(Figure):
    def __init__(self, root, idf, del_cbk):
        super().__init__(root, idf, del_cbk, "pt")
        self.vec = []
        for j in range(3):
            e = tk.Entry(self.frame)
            e.grid(row=1, column=j)
            self.vec.append(e)

    def get_as_str(self):
        s = ""
        for j in range(3):
            s += format_float(self.vec[j].get()) + ", "
        #s = s[:-1]
        return s


class Ln(Figure):
    def __init__(self, root, idf, del_cbk):
        super().__init__(root, idf, del_cbk, "ln")
        self.pts = []
        for p in range(2):
            vec = []
            for j in range(3):
                e = tk.Entry(self.frame)
                e.grid(row=1+p, column=j)
                vec.append(e)
            self.pts.append(vec)

    def get_as_str(self):
        s = ""
        for i in range(2):
            for j in range(3):
                s += format_float(self.pts[i][j].get()) + ", "
            s += "\n"
        #s = s[:-1]
        return s


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
            s += format_float(self.vec[j].get()) + ", "
        #s = s[:-1]
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
                s += format_float(self.mat[i][j].get()) + ", "
            s += "\n"
#        s = s[:-1]
        return s


class Param:
    def __init__(self, rootframe, name, cbk, del_cbk):
        self.name = name
        self.frame = tk.Frame(rootframe)
        self.frame.pack(anchor=tk.S)
        self.frame.columnconfigure(1, weight=1)
        self.scale = tk.Scale(
            self.frame,
            label=self.name,
            from_=-1.0,
            to=1.0,
            digits=2,
            resolution=0.125,
            orient=tk.HORIZONTAL,
            command=cbk
        )
        self.scale.grid(row=0, column=1, sticky=tk.EW)
        self.x_btn = tk.Button(
            self.frame,
            text="X",
            fg="#FFFFFF",
            bg="#DB0000",
            activeforeground="#000000",
            activebackground="#FF2400",
            command=del_cbk
        )
        self.x_btn.grid(row=0, column=3)
        self.lo_entry = tk.Entry(self.frame, width=4)
        self.lo_entry.bind("<Return>", self.update_lo)
        self.lo_entry.insert(tk.END, "-1")
        self.lo_entry.grid(row=0, column=0, sticky=tk.SE)
        self.hi_entry = tk.Entry(self.frame, width=4)
        self.hi_entry.bind("<Return>", self.update_hi)
        self.hi_entry.insert(tk.END, "+1")
        self.hi_entry.grid(row=0, column=2, sticky=tk.SW)

    def get(self):
        return self.scale.get()

    def update_lo(self, *args):
        try:
            self.scale.configure(from_=float(self.lo_entry.get()))
        except:
            print("enter some numba u idiot")

    def update_hi(self, *args):
        try:
            self.scale.configure(to=float(self.hi_entry.get()))
        except:
            print("enter some numba u idiot")
