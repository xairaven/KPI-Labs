import numpy as np
from scipy.integrate import solve_ivp

import printer

# Consts for text formatting
LIB = "[LIB]"
METHOD = "Euler's"

# ODE System Variables
G = 0
K = 0


def start(g, k, start_conditions, t_span, accuracy=0.1, SHORTENED_PRINT=False):
    init_ode_system_vars(g, k)

    # Solve using Euler's method
    solution_raw = solve_ivp(model, t_span, start_conditions, args=(G, K),
                             method='RK23', t_eval=np.linspace(0, 1))

    # Extract solutions
    t = solution_raw.t
    x = solution_raw.y[0]
    y = solution_raw.y[1]

    printer.show(LIB, METHOD, t, x, y, SHORTENED_PRINT)


# Define G and K in module
def init_ode_system_vars(g, k):
    global G
    global K
    G = g
    K = k


# Define the system of differential equations
def model(t, cond, g, k):
    x, y = cond
    dxdt = k * t + x - y + g
    dydt = -x + k * y
    return [dxdt, dydt]
