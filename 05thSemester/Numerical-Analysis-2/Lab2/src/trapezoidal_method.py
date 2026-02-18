import utils


def start(function, x_range, accuracy):
    h = utils.get_delta(algorithm, function, accuracy)
    steps = utils.get_steps(h, x_range)

    result = algorithm(function, x_range, h, steps)
    print(f"Method: Trapezoidal\n"
          f"Steps: {steps:d}\n"
          f"Length of step: {h:.4f}\n"
          f"Result: {result:.8f}\n")


# Implementation of trapezoidal method
def algorithm(function, x_range, h, steps):
    lo, hi = x_range

    # Finding sum
    integration = function(lo) + function(hi)

    for i in range(1, steps):
        k = lo + i * h
        integration = integration + 2 * function(k)

    # Finding final integration value
    integration = integration * h / 2

    return integration
