def show(lib, method, t, x, y, shortened=False):
    print(f"*----------*---------*--------------*--------------*--------------*--------------*")
    print(f"|          |  STEP   |    METHOD    |      T       |      X       |       Y      |")
    print(f"*----------*---------*--------------*--------------*--------------*--------------*")

    indexes = list(range(1, len(t) + 1))

    if shortened:
        t = [t[0], t[-1]]
        x = [x[0], x[-1]]
        y = [y[0], y[-1]]
        indexes = [indexes[0], indexes[-1]]

    length = len(t)
    for i in range(length):
        print(f"| {lib:^8s} | {indexes[i]:^7d} | {method:^12s} | {t[i]:^12.4f} | {x[i]:^12.4f} | {y[i]:^12.4f} |")

    print(f"*----------*---------*--------------*--------------*--------------*--------------*")
