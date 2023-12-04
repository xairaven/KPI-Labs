import printer
import utils

# Consts for text formatting
LIB = "[USER]"
METHOD = "Euler's"

# ODE System Variables
G = 0
K = 0


def start(g, k, start_conditions, t_span, accuracy=0.1, SHORTENED_PRINT=False):
    init_ode_system_vars(g, k)

    h = utils.get_delta(accuracy, algorithm)
    steps = utils.get_steps(h, t_span)

    T, X, Y = algorithm(start_conditions, t_span[0], h, steps)

    printer.show(LIB, METHOD, T, X, Y, SHORTENED_PRINT)


# Define G and K in module
def init_ode_system_vars(g, k):
    global G
    global K
    G = g
    K = k


# Define the system of differential equations
def model(t, variables):
    x, y = variables
    dxdt = K * t + x - y + G
    dydt = -x + K * y
    return [dxdt, dydt]


def algorithm(base_system, t, h, steps):
    # Variables for logging
    T_LOG = []
    X_LOG = []
    Y_LOG = []

    system = base_system.copy()

    start_t = t

    F = model(t, system)

    # LOGGING
    T_LOG.append(t), X_LOG.append(system[0]), Y_LOG.append(system[1])

    for step in range(1, steps + 1):
        for counter in range(0, len(system)):
            system[counter] = system[counter] + h * F[counter]
        t = start_t + step * h
        F = model(t, system)

        # LOGGING
        T_LOG.append(t), X_LOG.append(system[0]), Y_LOG.append(system[1])

    return [T_LOG, X_LOG, Y_LOG]
