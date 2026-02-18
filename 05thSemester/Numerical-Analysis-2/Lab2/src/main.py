import math

import json_loader
import trapezoidal_method

# GLOBAL VARIABLES (DEPENDING ON TASK)
settings = json_loader.get_data("settings.json")
X_RANGE = settings["Equation"]["X Range"]
ACCURACY = settings["Solving Parameters"]["Accuracy"]


def function(x):
    return x ** 2 * math.sqrt(1 - x ** 2)


# --- PROGRAM ---
trapezoidal_method.start(function, X_RANGE, ACCURACY)
print()