#!/usr/bin/env python3

import projec_p2 # rust lib
import tkinter as tk
import PIL
from PIL import ImageTk
import figures
import re

# global projec_p2 object

r_size = 400
r = projec_p2.ProjWrap(r_size)

# tk gui

rootwin = tk.Tk()
rootwin.configure(bg="#b6b6aa")
rootwin.geometry("600x400")
rootwin.title("Projec P2")
#icon = tk.PhotoImage(file="./icon16.png")
#rootwin.iconphoto(True,icon)


tk_img = PIL.ImageTk.PhotoImage(PIL.Image.frombuffer(
    mode="L",
    size=(r_size, r_size),
    data=bytes(r.get_pix_buff())
))
proj_lab = tk.Label(rootwin, image = tk_img)
proj_lab.pack(anchor=tk.NW, side=tk.LEFT)

right_frame = tk.Frame(rootwin)
right_frame.pack(anchor=tk.NW, expand=True, fill=tk.X, side=tk.LEFT)

prm_entry = tk.Entry(right_frame)
prm_entry.pack(anchor=tk.S)
prm_buton = tk.Button(
    right_frame,
    text="NEW\nPARAM",
    command=lambda: add_par(prm_entry.get())
)
prm_buton.pack()
prm_frame = tk.Frame(right_frame)
prm_frame.pack(anchor=tk.S)

entries = [] # tk.Entry objects will be stored dynamically
params  = {} # tk.Scale objects for parameters

def gen_entry_idf():
    idf = 0
    if len(entries) > 0:
        idf = entries[-1].idf + 1
    return idf

def add_fig(fig_type):
    idf = gen_entry_idf()
    if   fig_type == "eq":
        fig = figures.Eq(right_frame, idf, lambda: del_fig(idf))
    elif fig_type == "cn":
        fig = figures.Cn(right_frame, idf, lambda: del_fig(idf))
    else:
        raise Error
    entries.append(fig)

def del_fig(idf):
    e = None
    for i in range(len(entries)):
        if entries[i].idf == idf:
            e = entries.pop(i)
            break
    if e is None:
        raise IndexError
    else:
        e.frame.destroy() # cal destruir lo de dins?

# add parameter Scale widget
def add_par(name):
    if name in params:
        raise KeyError
    if name == "":
        print("embty srting u idiot")
        raise Error
    s = tk.Scale(
        prm_frame,
        label=name,
        from_=-1.0,
        to=+1.0,
        resolution=0.125,
        orient=tk.HORIZONTAL,
        command=lambda v: update_tk_img()
    )
    s.pack()
    params[name] = s

# delete parameter Object
def del_par():
    raise NotImplementedError

def update_tk_img():
    draw_tk_img()
    tk_img = PIL.ImageTk.PhotoImage(PIL.Image.frombuffer(
        mode="L",
        size=(r_size, r_size),
        data=bytes(r.get_pix_buff())
    ))
    proj_lab.image = tk_img
    proj_lab.configure(image=tk_img)
'''
    for e in entries:
        r.draw_taco
'''

def preproc_taco():
    taco = ""
    for fig in entries:
        if isinstance(fig, figures.Eq):
            taco += "eq\n"
        elif isinstance(fig, figures.Cn):
            taco += "cn\n"
        taco += fig.get_as_str() + "\n"
    for (prm, scl) in params.items():
        taco = re.sub(r"\b" + prm + r"\b", "{:.4f}".format(scl.get()), taco)
    return taco

def draw_tk_img():
    global tk_img
    taco = preproc_taco()
    print(taco)
    r.reset()
    try:
        r.draw_taco(taco)
    except:
        print("error at draw_taco")

b4 = tk.Button(
    master=right_frame,
    text="TACO",
#    bg="#DBDBFF",
    command=update_tk_img,
    width=10,
    pady=6
)
b4.pack(anchor=tk.N)


entry_type = tk.StringVar()
rb_frame = tk.Frame(right_frame)
rb_frame.pack(anchor=tk.N)

rb_ln = tk.Radiobutton(
    rb_frame,
    text="line",
    variable=entry_type,
    value="ln"
)
rb_ln.pack(anchor=tk.W)
rb_eq = tk.Radiobutton(
    rb_frame,
    text="equation",
    variable=entry_type,
    value="eq"
)
rb_eq.pack(anchor=tk.W)
rb_cn = tk.Radiobutton(
    rb_frame,
    text="conic",
    variable=entry_type,
    value="cn"
)
rb_cn.pack(anchor=tk.W)

be = tk.Button(
    master=right_frame,
    text="NEW\nENTRI",
#    bg="#DBDBFF",
    command=lambda:add_fig(entry_type.get()),
    width=10,
    pady=6
)
be.pack(anchor=tk.N)

rootwin.mainloop()
