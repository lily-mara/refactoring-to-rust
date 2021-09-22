import sys
import rust_json

s = 0

for line in sys.stdin:
  s += rust_json.sum(line)

print(s)
