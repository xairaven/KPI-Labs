import printer
import utils

# Consts for text formatting
LIB = "[USER]"
METHOD = "Runge-Kutta"

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

    # LOGGING
    T_LOG.append(t), X_LOG.append(system[0]), Y_LOG.append(system[1])

    for step in range(1, steps + 1):
        F = model(t, system)

        # K0
        k0 = [0] * len(system)
        for i in range(0, len(F)):
            k0[i] = h * F[i]

        # K1
        k1 = [0] * len(system)
        first_member = t + h / 2
        second_member = [None] * len(system)
        for i in range (0, len(k0)):
            second_member[i] = system[i] + k0[i] / 2
        F = model(first_member, second_member)
        for i in range(0, len(F)):
            k1[i] = h * F[i]

        # K2
        k2 = [0] * len(system)
        first_member = t + h / 2
        second_member = [None] * len(system)
        for i in range(0, len(k1)):
            second_member[i] = system[i] + k1[i] / 2
        F = model(first_member, second_member)
        for i in range(0, len(F)):
            k2[i] = h * F[i]

        # K3
        k3 = [0] * len(system)
        first_member = t + h
        second_member = [None] * len(system)
        for i in range(0, len(k2)):
            second_member[i] = system[i] + k2[i]
        F = model(first_member, second_member)
        for i in range(0, len(F)):
            k3[i] = h * F[i]

        # SYSTEM
        t = start_t + step * h
        for counter in range(0, len(system)):
            system[counter] = system[counter] + (k0[counter] + 2*k1[counter] + 2*k2[counter] + k3[counter])/6

        # LOGGING
        T_LOG.append(t), X_LOG.append(system[0]), Y_LOG.append(system[1])

    return [T_LOG, X_LOG, Y_LOG]
