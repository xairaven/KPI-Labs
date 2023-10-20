def validate(table):
    for row in table:
        for value in row:
            if value is None:
                raise ArithmeticError("Columns have different amount of numbers")


def solve(coeffs, table):
    model_values = []

    for row in table:
        value = coeffs[0]
        i = 1

        for cell_value in row:
            value += coeffs[i] * cell_value
            i += 1

        model_values.append(value)

    return model_values
