import tkinter as tk
from tkinter import ttk
from matplotlib.backends.backend_tkagg import FigureCanvasTkAgg
import matplotlib.pyplot as plt
import numpy as np
from funcs import funcs

def update_plot(event=None):
    xmax = 10**float(xmax_spinner_var.get())  # Read xmax from the spinner
    x = np.linspace(0, xmax, 1000)  # xmin is set to 0
    for i, (label, func) in enumerate(funcs):
        coefficient = float(spinner_var[i].get())
        y = coefficient * func(x)
        lines[i].set_xdata(x)
        lines[i].set_ydata(y)

    ymax = 10**float(ymax_spinner_var.get())  # Read ymax from the spinner
    ax.set_xlim(0, xmax)
    ax.set_ylim(0, ymax)

    canvas.draw()





def on_closing():
    root.destroy()

root = tk.Tk()
root.title("Function Plotter")

# Handle window close event
root.protocol("WM_DELETE_WINDOW", on_closing)

# Create Matplotlib figure and canvas
fig, ax = plt.subplots(figsize=(6, 6))
canvas = FigureCanvasTkAgg(fig, master=root)
canvas.get_tk_widget().grid(row=0, column=5, rowspan=20, sticky="nsew")

# Create 30 spinner boxes with increment and decrement arrows
spinner_var = [tk.DoubleVar() for _ in range(30)]
spinners = []

# Function to update plot when spinner value changes
def update_plot_on_arrow_click(event):
    update_plot()

# Organize the spinners into 3 columns and 20 rows
for i, (label, func) in enumerate(funcs):
    tk.Label(root, text=label).grid(row=(i % 10) * 2, column=(i // 10) * 2, sticky="w")
    spinner = ttk.Spinbox(root, from_=-1000.0, to=1000.0, textvariable=spinner_var[i], increment=0.1, width=10)
    spinner.grid(row=(i % 10) * 2 + 1, column=(i // 10) * 2, sticky="w")
    spinners.append(spinner)

    # Bind the update_plot function to the FocusOut and ButtonRelease-1 events of each spinner
    spinner.bind("<FocusOut>", update_plot)
    spinner.bind("<ButtonRelease-1>", update_plot_on_arrow_click)

# Initialize spinner values to 0
for var in spinner_var:
    var.set(0.0)

# Initialize the plot with 30 functions
xmax = 10.0
ymax = 10.0

x = np.linspace(0, xmax, 1000)  # xmin is set to 0
lines = [ax.plot(x, func(x))[0] for label, func in funcs]

# Configure row and column weights for resizing
for i in range(20):
    root.grid_rowconfigure(i, weight=1)
for i in range(6):
    root.grid_columnconfigure(i, weight=1)

# Create 4 spinners for controlling xmax and ymax
xmax_spinner_var = tk.DoubleVar()
xmax_spinner = ttk.Spinbox(root, from_=-10.0, to=10.0, textvariable=xmax_spinner_var, increment=1.0, width=10)
xmax_spinner.grid(row=0, column=6, sticky="w")
ymax_spinner_var = tk.DoubleVar()
ymax_spinner = ttk.Spinbox(root, from_=-10.0, to=10.0, textvariable=ymax_spinner_var, increment=1.0, width=10)
ymax_spinner.grid(row=2, column=6, sticky="w")

# Label for xmax and ymax spinners
tk.Label(root, text="X-Max").grid(row=1, column=6, sticky="w")
tk.Label(root, text="Y-Max").grid(row=3, column=6, sticky="w")

# Initialize xmax and ymax spinners to 0
xmax_spinner_var.set(0)
ymax_spinner_var.set(0)

# Bind the update_plot function to the FocusOut event of the new spinners
xmax_spinner.bind("<FocusOut>", update_plot)
ymax_spinner.bind("<FocusOut>", update_plot)# Bind the update_plot function to the ButtonRelease-1 event of the xmax and ymax spinners
xmax_spinner.bind("<ButtonRelease-1>", update_plot)
ymax_spinner.bind("<ButtonRelease-1>", update_plot)


# Run the Tkinter main loop
root.mainloop()
