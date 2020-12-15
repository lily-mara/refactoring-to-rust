import random

with open('data.csv', 'w') as f:
    for _ in range(1_000_000):
        line = ','.join(str(random.randint(0,256)) for _ in range(100))
        f.write(f'{line}\n')