def get_steps(h, x_range):
    lo, hi = x_range
    return round((hi - lo) / h)


def get_delta(algorithm, function, accuracy):
    MIN_H = 0.00001
    MAX_H = 1

    DEFAULT_X_RANGE = [0, 1]

    h = 0.1
    while MAX_H >= h > MIN_H:

        result1 = algorithm(function, DEFAULT_X_RANGE, h, get_steps(h, DEFAULT_X_RANGE))

        h = h / 2
        result2 = algorithm(function, DEFAULT_X_RANGE, h, get_steps(h, DEFAULT_X_RANGE))

        if abs(result1 - result2) < accuracy:
            return h

    return h * 2
