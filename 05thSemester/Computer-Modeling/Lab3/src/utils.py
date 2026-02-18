def get_steps(h, t_span):
    t_start, t_end = t_span
    return round((t_end - t_start) / h)


def get_delta(accuracy, algorithm):
    BASE_SYSTEM = [0, 0]
    BASE_T = 0

    MIN_H = 0.00001
    MAX_H = 0.1

    h = 0.1
    while MAX_H >= h > MIN_H:
        T, X, Y = algorithm(BASE_SYSTEM, BASE_T, h, get_steps(h, [0, 1]))
        x1 = X[len(X) - 1]
        y1 = Y[len(Y) - 1]

        h = h / 2
        T, X, Y = algorithm(BASE_SYSTEM, BASE_T, h, get_steps(h, [0, 1]))
        x2 = X[len(X) - 1]
        y2 = Y[len(Y) - 1]

        if abs(x1 - x2) < accuracy and abs(y1 - y2) < accuracy:
            return h

    return h * 2
