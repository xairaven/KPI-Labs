import numpy as np
from scipy.integrate import odeint
from scipy.optimize import fsolve


# Initial system. Returns values of derivatives.
# Used in "objective" method for solving this system.
def model(y, t, b, x, coefficients):
    b1, b2 = b
    g, k = coefficients
    y1, y2 = y

    dy1dt = -k * b1 * y1 - 2 * x * b2 * y2 + np.cos(t)
    dy2dt = -x * b1 * y1 - 4 * g * b2 * y2 + k * np.sin(t)
    return [dy1dt, dy2dt]


# Calculates difference between solved Y values and Y values from experiment.
# Used in "calculate" method.
def objective(b, t, variables, coefficients):
    t_span = np.array([0, t])  # Time span from 0 to 1
    x, y1, y2 = variables

    y_pred = odeint(model, [y1, y2], t_span, args=(b, x, coefficients))
    y_actual = np.array([y1, y2])  # Given values of y1 and y2
    return y_pred[-1] - y_actual


# T - fixed number.
# Data - arrays of experimental data from task (X, Y1, Y2).
# fsolve function minimize differences between Y. Seeks for Bs that do it.
def calculate(t, arrays, coefficients):
    x, y1, y2 = arrays

    # Arrays lengths must be equal
    rows = len(arrays[0])
    for array in arrays:
        if len(array) != rows:
            raise ValueError("There are different amount of rows in arrays")

    # Initial guess for Beta1 and Beta2
    initial_guess = np.array([1, 1])
    result = [0, 0]

    # Finding Betas from every experiment
    for i in range(0, rows):
        row = [x[i], y1[i], y2[i]]

        # Getting betas
        b_solution = fsolve(objective, initial_guess, args=(t, row, coefficients))

        # Result - array of average betas
        for j in range(len(b_solution)):
            result[j] += b_solution[j] / rows

        # Printing pairs
        print(f"Pair {i + 1}: [{b_solution[0]}, {b_solution[1]}]")

    # Printing results
    print(f"Result: {result}")
    return result
