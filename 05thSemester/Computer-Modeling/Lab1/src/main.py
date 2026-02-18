import sys

import xlsx_service
import solver_service


def main():
    if len(sys.argv) != 2:
        raise ValueError("There must be 1 argument: path to .xlsx file")

    excel_file_name = sys.argv[1]

    table = xlsx_service.get_table(excel_file_name)
    solver_service.validate(table)

    coeffs = []
    for i in range(0, len(table[0]) + 1):
        coef = float(input(f"b{i}: "))
        coeffs.append(coef)

    model_values = solver_service.solve(coeffs, table)
    xlsx_service.write_table(".\\Data\\results.xlsx", model_values)
    print("Results were written successfully")


if __name__ == '__main__':
    main()
