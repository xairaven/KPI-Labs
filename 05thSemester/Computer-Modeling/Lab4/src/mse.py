import math


def calculate(current, precise):
    summa = 0

    n = len(current)
    for i in range(0, n):
        summa += (current[i] - precise[i]) ** 2

    return math.sqrt(summa / (n - 1))
