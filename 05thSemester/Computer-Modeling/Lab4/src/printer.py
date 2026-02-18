from prettytable import PrettyTable


def show(arrays, headers, title=""):
    table = PrettyTable()

    if title != "":
        table.title = title

    # If there's headers, we'll print it
    if len(headers) != 0:
        table.field_names = headers

    # Arrays lengths must be equal
    rows = len(arrays[0])
    for array in arrays:
        if len(array) != rows:
            raise ValueError("There are different amount of rows in arrays")

    # Adding rows
    for counter in range(0, rows):
        row = []
        for array in arrays:
            element = array[counter]

            # If there are floats, format it
            if isinstance(element, float):
                element = "{:.5f}".format(element)

            row.append(element)

        table.add_row(row)

    print(table)
