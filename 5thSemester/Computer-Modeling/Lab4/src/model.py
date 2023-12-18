import math

import numpy as np
from scipy.integrate import solve_ivp


# Parameters: beta1, beta2, ...
# Coefficients: G, K (Fixed constants for task. Given in task specification)
# T Span - range of Ts (for example - [0, 1]
def runge_kutta(x, t_span, parameters, coefficients):
    # y1, y2 = 0
    start_conditions = [0, 0]

    # Solve IVP - library method for solving system by Runge-Kutta method
    solution = solve_ivp(model, t_span, start_conditions, args=(x, parameters, coefficients),
                         method='RK45', t_eval=np.linspace(0, 1))

    # Extract solutions
    t = solution.t
    y1 = solution.y[0]
    y2 = solution.y[1]

    # Return arrays
    return [t, y1, y2]


# Made for optimization purposes.
# Returns last Y1 and Y2 values.
def runge_kutta_precised(x, t_span, parameters, coefficients):
    # y1, y2 = 0
    start_conditions = [0, 0]

    # Solve IVP - library method for solving system by Runge-Kutta method
    solution = solve_ivp(model, t_span, start_conditions, args=(x, parameters, coefficients),
                         method='RK45', t_eval=np.linspace(0, 1))

    y1 = solution.y[0]
    y2 = solution.y[1]

    # Return arrays
    return [y1[-1], y2[-1]]


# Model function calculates variables from system. So, it's system defining function.
# Variables - y1, y2
# Parameters - beta1, beta 2
# Coefficients (G, K) - Fixed constants for task. Given in task specification
def model(t, variables, x, parameters, coefficients):
    y1, y2 = variables
    b1, b2 = parameters
    g, k = coefficients

    y1_derivative = -k * b1 * y1 - 2 * x * b2 * y2 + math.cos(t)
    y2_derivative = -x * b1 * y1 - 4 * g * b2 * y2 + k * math.sin(t)
    return [y1_derivative, y2_derivative]
