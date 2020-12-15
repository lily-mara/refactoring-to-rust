import time


with open('data.csv') as f:
    data = f.read()

start = time.time()
value = sum_csv_column(data, 10)
end = time.time()

print(end - start)

print(value)
