import json


def get_data(file_name):
    try:
        f = open(file_name)
        data = json.load(f)
        f.close()
    except Exception as e:
        print(f"There's no {file_name}. Process terminated")
        exit(1)
    else:
        return data