import sys
import json

s = 0

for line in sys.stdin:
  value = json.loads(line)
  s += value['value']
  s += len(value['name'])

print(s)
