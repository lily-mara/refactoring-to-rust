import sys

import rust_json

def sum(lines_iter):
  s = 0

  for line in lines_iter:
    s += rust_json.sum(line)

  return s

if __name__ == '__main__':
  print(sum(sys.stdin))
