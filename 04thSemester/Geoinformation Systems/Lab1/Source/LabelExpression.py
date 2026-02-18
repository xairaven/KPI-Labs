def FindLabel ([COUNTRY], [POP_1994]):
    list = ["Chile", "Argentina", "Bolivia", "Peru"]
    if [COUNTRY] in list:
        return [COUNTRY] + "\n" + str([POP_1994])
    else:
        return ""