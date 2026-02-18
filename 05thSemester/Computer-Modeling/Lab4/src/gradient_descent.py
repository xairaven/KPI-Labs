import model
import mse


def calculate_parameters(data, coefficients, init_parameters, max_steps, precision):
    error = 9999999.0
    previous_error = error
    iteration = 1

    step = 0.000001

    gradient_params = []

    b1, b2 = init_parameters

    while True:
        if error < precision:
            print(f"*** Error < Precision ({error:.9f} < {precision:.9f}) ***)")
            break
        elif iteration > max_steps:
            print(f"*** Iteration limit is over! ({iteration} | {max_steps}) ***")
            break

        print(f"{iteration} | Error: {error:0.9f} | B1: {b1:0.5f} | B2: {b2:0.9f}")

        temp_previous_error = previous_error

        previous_error = error
        error = calc_error(coefficients, data, [b1, b2])

        if previous_error < error:
            error = previous_error
            previous_error = temp_previous_error

            b1 += step * gradient_params[0]
            b2 += step * gradient_params[1]

            step /= 2
            continue

        if previous_error == error:
            break

        gradient_params = solve_gradient(coefficients, data, [b1, b2])
        b1 -= step * gradient_params[0]
        b2 -= step * gradient_params[1]

        iteration += 1

    print(f"Result: {[b1, b2]}")
    return b1, b2


def calc_error(coefficients, data, parameters):
    # Fixed value. Got from task
    t_span_fixed = [0, 1]

    # Arrays
    x_array, y1_observed, y2_observed = data

    y1_predicted = []
    y2_predicted = []

    for x in x_array:
        y1, y2 = model.runge_kutta_precised(x, t_span_fixed, parameters, coefficients)
        y1_predicted.append(y1)
        y2_predicted.append(y2)

    error = mse.calculate(y1_predicted, y1_observed) + mse.calculate(y2_predicted, y2_observed)
    return error


def solve_gradient(coefficients, data, params):
    return [solve_gradient_1(coefficients, data, params),
            solve_gradient_2(coefficients, data, params)]


def solve_gradient_1(coefficients, data, params):
    e = 0
    iteration = 0

    # Fixed value. Got from task
    t_span_fixed = [0, 1]

    x, y1_exp, y2_exp = data
    for i in range(len(x)):
        y1, y2 = model.runge_kutta_precised(x[i], t_span_fixed, params, coefficients)

        delta1 = calc_delta_1(coefficients, data, params, iteration)
        iteration += 1

        e += (y1 - y1_exp[i]) * delta1[0] + (y2 - y2_exp[i]) * delta1[1]

    return e


def solve_gradient_2(coefficients, data, params):
    e = 0
    iteration = 0

    # Fixed value. Got from task
    t_span_fixed = [0, 1]

    x, y1_exp, y2_exp = data
    for i in range(len(x)):
        y1, y2 = model.runge_kutta_precised(x[i], t_span_fixed, params, coefficients)

        delta2 = calc_delta_2(coefficients, data, params, iteration)
        iteration += 1

        e += (y1 - y1_exp[i]) * delta2[0] + (y2 - y2_exp[i]) * delta2[1]

    return e


def calc_delta_1(coefficients, data, params, iteration):
    db = 0.000001

    # Fixed value. Got from task
    t_span_fixed = [0, 1]

    b1, b2 = params
    x, y1_exp, y2_exp = data
    y1, y2 = model.runge_kutta_precised(x[iteration], t_span_fixed, [b1 + db, b2], coefficients)
    y11, y21 = model.runge_kutta_precised(x[iteration], t_span_fixed, [b1, b2], coefficients)

    result = [(y1 - y11) / db, (y2 - y21) / db]

    return result


def calc_delta_2(coefficients, data, params, iteration):
    db = 0.000001

    # Fixed value. Got from task
    t_span_fixed = [0, 1]

    b1, b2 = params
    x, y1_exp, y2_exp = data
    y1, y2 = model.runge_kutta_precised(x[iteration], t_span_fixed, [b1, b2 + db], coefficients)
    y11, y21 = model.runge_kutta_precised(x[iteration], t_span_fixed, [b1, b2], coefficients)

    result = [(y1 - y11) / db, (y2 - y21) / db]

    return result
