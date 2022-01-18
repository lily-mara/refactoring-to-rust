import sys
import json

def sum(lines_iter):
  s = 0

  for line in lines_iter:
    value = json.loads(line)
    s += value['value']
    s += len(value['name'])

  return s

if __name__ == '__main__':
  print(sum(sys.stdin))
