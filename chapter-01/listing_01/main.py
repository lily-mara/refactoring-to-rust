import time


def sum_csv_column(data, column):
  sum = 0

  for line in data.split("\n"):
    if len(line) == 0:
      continue

    value_str = line.split(",")[column]
    sum += int(value_str)

  return sum


with open('data.csv') as f:
    data = f.read()

start = time.time()
value = sum_csv_column(data, 10)
end = time.time()

print(end - start)

print(value)
