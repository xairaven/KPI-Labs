import sys
import random

DIM = int(sys.argv[1])
PATH = sys.argv[2]

with open(PATH, 'w') as file:
    file.write(str(DIM) + '\n')
    for _ in range(DIM):
        numbers = ' '.join(str(random.randint(0, 50)) for _ in range(DIM))
        file.write(numbers + '\n')
