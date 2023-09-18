from functools import reduce

import openpyxl


def get_column(excel_filename, column_letter, start_from=1):
    column = openpyxl.load_workbook(excel_filename).active[column_letter]

    data = []
    for index in range(start_from, len(column)):
        cell = column[index]
        data.append(cell.value)

    return data


def get_table(excel_filename, columns_data, start_from=1):
    sheet = openpyxl.load_workbook(excel_filename).active

    # Getting list of each specified column. Column - a list of cells
    columns = list(map(lambda letter: sheet[letter], columns_data.values()))

    table = []
    max_column_length = max([len(column) for column in columns])
    for index in range(start_from, max_column_length):
        # Creating list with row.values
        row = list(map(lambda column: column[index].value, columns))

        # Create dict with keys as column headers and values as row cells values
        table.append(dict(zip(columns_data.keys(), row)))

    return table

