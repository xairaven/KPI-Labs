import json_loader
import lib_euler
import lib_runge_kutta
import my_euler
import my_runge_kutta

# GLOBAL VARIABLES (DEPENDING ON TASK)
settings = json_loader.get_data("settings.json")

G = settings["ODE Variables"]["G"]
K = settings["ODE Variables"]["K"]

START_VALUES = settings["System"]["Start Values"]
T_SPAN = settings["System"]["T Range"]

ACCURACY = settings["Solving Parameters"]["Accuracy"]

SHORTENED_PRINT = settings["Printing Parameters"]["Shortened"]

# --- PROGRAM ---
my_euler.start(G, K, START_VALUES.copy(), T_SPAN, ACCURACY, SHORTENED_PRINT=SHORTENED_PRINT)
print()

lib_euler.start(G, K, START_VALUES.copy(), T_SPAN, SHORTENED_PRINT=SHORTENED_PRINT)
print()

my_runge_kutta.start(G, K, START_VALUES.copy(), T_SPAN, ACCURACY, SHORTENED_PRINT=SHORTENED_PRINT)
print()

lib_runge_kutta.start(G, K, START_VALUES.copy(), T_SPAN, SHORTENED_PRINT=SHORTENED_PRINT)
print()

