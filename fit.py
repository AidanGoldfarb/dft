import numpy as numpy
from scipy.optimize import curve_fit
import matplotlib.pyplot as pyplot
import glob
import os

DIR = os.getcwd()

def funcA(x,a):
    return a*numpy.sqrt(x*x)

def funcB(x,a):
    return numpy.sqrt(a*x*x)

def parse():
    x = []
    y_time = []
    y_trace = []
    with open(DIR + "/fft_output.txt") as file:
        next(file)
        for line in file:
            arr = line.split("\t")
            x.append(float(arr[0]))
            y_time.append(float(arr[1]))
            y_trace.append(float(arr[2]))
    return (x,y_time,y_trace)

def graph(x,y,func,title,xaxis_lab, yaxis_lab,filename):
    popt, pcov = curve_fit(func, x, y)

    print("A: ", popt)
    print("Covar: ", pcov)

    trialX = numpy.linspace(x[0], x[-1], 1000)
    ySqrt = func(trialX, *popt)

    pyplot.figure()
    pyplot.title("a = " + str(popt), fontsize=9)
    pyplot.suptitle(title)
    pyplot.xlabel(xaxis_lab)
    pyplot.ylabel(yaxis_lab)
    pyplot.plot(x, y, label='Data', marker='o')
    pyplot.plot(trialX, ySqrt, 'r-',ls='--', label="sqrt Fit")
    # pyplot.plot(trialX,   y, label = '10 Deg Poly')
    pyplot.legend()
    # pyplot.show()
    pyplot.savefig(filename+".png", bbox_inches = 'tight')


(x,y_time,y_trace) = parse() #[4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192]

graph(x,y_trace, funcA, "Trace length","matrix size","DMD / trace_len", "trace_A")

graph(x,y_trace, funcB, "Trace length","matrix size","DMD / trace_len", "trace_B")

graph(x,y_time, funcA, "Time complexity","matrix size","DMD / time cmplx", "time_A")

graph(x,y_time, funcB, "Time complexity","matrix size","DMD / time cmplx", "time_B")