import gradient_descent
import json_loader
import plot
import model
import printer
import initial_parameters

# GLOBAL VARIABLES (DEPENDING ON TASK)
settings = json_loader.get_data("settings.json")

g = settings["ODE Variables"]["G"]
k = settings["ODE Variables"]["K"]

t_fixed = settings["Experiments"]["T"]

coefficients = [g, k]
t_span = settings["System"]["T Range"]

# Three arrays: X, Y1, Y2. Experimental data.
experimental_data = [
    [2 * k + g, 2 * k + 0.9 * g, 2 * k + 1.1 * g, 2 * k + 0.95 * g],
    [k + 0.1 * g, k + 0.2 * g, k - 0.1 * g, k + 0.15 * g],
    [g, 1.2 * g, 0.9 * g, 1.1 * g]
]


# Menu option 1
def calculate_initial_parameters():
    initial_parameters.calculate(t_fixed, experimental_data.copy(), coefficients)


# Menu option 2
def parametric_identification():
    b1 = float(input("B1: "))
    b2 = float(input("B2: "))

    max_steps = int(input("Max amount of steps: "))
    error = float(input("Precision: "))

    gradient_descent.calculate_parameters(experimental_data.copy(), coefficients, [b1, b2], max_steps, error)


# Menu option 3
def calculate_model():
    x = float(input("X: "))
    b1 = float(input("B1: "))
    b2 = float(input("B2: "))

    parameters = [b1, b2]
    t, y1, y2 = model.runge_kutta(x, t_span, parameters, coefficients)

    # Print results
    printer.show(arrays=[t, y1, y2], headers=["T", "Y1", "Y2"])

    # Show graph
    plot.show(t, y1, y2)


# Menu option 4
def compare_model_vs_experiments():
    b1 = float(input("B1: "))
    b2 = float(input("B2: "))

    x, y1, y2 = experimental_data

    # X, Y1, Y2 arrays have the same length. So, we can take X length
    experiments_count = len(x)

    print()

    x_general = []
    y1_general = []
    y2_general = []
    parameters = [b1, b2]

    for i in range(0, experiments_count):
        t, y1, y2 = model.runge_kutta(x[i], t_span, parameters, coefficients)
        x_general.append(x[i]), y1_general.append(y1[-1]), y2_general.append(y2[-1])

    printer.show(arrays=[x_general, y1_general, y2_general], headers=["X", "Y1", "Y2"], title="Model Data")

    print()

    # Show data from task (COMPARING)
    printer.show(experimental_data, ["X", "Y1", "Y2"], title="Experimental data")
    _, y1_exp, y2_exp = experimental_data

    # Sort Xs and Ys according to Xs
    x_general.sort()
    y1_general = [y1_general[1], y1_general[3], y1_general[0], y1_general[2]]
    y2_general = [y2_general[1], y2_general[3], y2_general[0], y2_general[2]]
    y1_exp = [y1_exp[1], y1_exp[3], y1_exp[0], y1_exp[2]]
    y2_exp = [y2_exp[1], y2_exp[3], y2_exp[0], y2_exp[2]]
    plot.show4(x_general, y1_general, y2_general, y1_exp, y2_exp)

    print()


run_program = True
while run_program:
    print("\n" +
          "MENU | Lab 4 | Kovalov Alex\n" +
          "1. Calculate the initial parameters\n" +
          "2. Parametric identification | Gradient Descent method\n" +
          "3. Calculate model | Fixed x, parameters | Plot Output\n" +
          "4. Comparison | Model vs Experimental Data\n" +
          "\n" +
          "9. Exit\n")

    choice = int(input("INPUT: "))
    match choice:
        # Calculate the initial parameters
        case 1:
            calculate_initial_parameters()

        # Parametric identification | Gradient Descent method
        case 2:
            parametric_identification()

        # Calculate model | Fixed x, parameters | Plot Output
        case 3:
            calculate_model()

        # Comparison | Model vs Experimental Data
        case 4:
            compare_model_vs_experiments()

        # Exit program
        case 9:
            print("Bye!")
            run_program = False

        case _:
            print("\n*** WRONG OPTION ***\n")
