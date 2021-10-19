import numpy as numpy
from scipy.optimize import curve_fit
import matplotlib.pyplot as pyplot


def func(x,a):
    return a*numpy.sqrt(x)

x = [4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192]
y = [3.2414997, 25.47816, 68.08528, 180.50806, 436.321, 1033.0516, 2367.469, 5370.2993, 11986.787, 26575.236, 58388.945, 127660.52]

popt, pcov = curve_fit(func, x, y)

print("A: ", popt)
print("Covar: ", pcov)

# pyplot.figure()
# pyplot.plot(xData, yData, label='Data', marker='o')
# pyplot.plot(trialX, yEXP, 'r-',ls='--', label="Exp Fit")
# pyplot.plot(trialX,   y, label = '10 Deg Poly')
# pyplot.legend()
# pyplot.show()