import sys
import random

def generate_random_numbers(num):
    random_numbers = [random.randint(0, num) for _ in range(num)]
    return random_numbers

def write_to_file(numbers, path):
    with open(path, "w") as file:
        file.write(str(len(numbers)) + " " + " ".join(map(str, numbers)))

if __name__ == "__main__":
    # Check if the number of command-line arguments is correct
    if len(sys.argv) != 3:
        print("Usage: python script.py <number_of_random_numbers> <path_to_output_file>")
        sys.exit(1)

    try:
        amount = int(sys.argv[1])
        path = sys.argv[2]
    except ValueError:
        print("Please provide a valid integer as the amount of random numbers.")
        sys.exit(1)

    random_numbers = generate_random_numbers(amount)
    write_to_file(random_numbers, path)
