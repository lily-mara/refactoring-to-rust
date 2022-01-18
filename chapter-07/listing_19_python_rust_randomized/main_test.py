import json
import string
import random

import main

def test_10_lines():
  lines = [
    '{ "name": "Stokes Baker", "value": 954832 }',
    '{ "name": "Joseph Solomon", "value": 279836 }',
    '{ "name": "Gonzalez Koch", "value": 140431 }',
    '{ "name": "Parrish Waters", "value": 490411 }',
    '{ "name": "Sharlene Nunez", "value": 889667 }',
    '{ "name": "Meadows David", "value": 892040 }',
    '{ "name": "Whitley Mendoza", "value": 965462 }',
    '{ "name": "Santiago Hood", "value": 280041 }',
    '{ "name": "Carver Caldwell", "value": 632926 }',
    '{ "name": "Tara Patterson", "value": 678175 }',
  ]

  assert main.sum(lines) == 6203958

def test_compare_py_rust(monkeypatch):
  compare_py_and_rust(
    monkeypatch,
    ['{ "name": "Stokes Baker", "value": 954832 }']
  )

def test_random_inputs(monkeypatch):
  for _ in range(100):
    randomized_test_case(monkeypatch)

def randomized_test_case(monkeypatch):
  number_of_lines = random.randint(100, 500)

  lines = []
  for _ in range(number_of_lines):
    number_of_chars = random.randint(100, 200)

    lines.append(json.dumps({
      'name': ''.join(random.choices(string.ascii_lowercase, k=number_of_chars)),
      'value': random.randint(0, 10_000),
    }))

  compare_py_and_rust(monkeypatch, lines)

def compare_py_and_rust(monkeypatch, input):
  rust_result = main.sum(input)

  def python_sum(line):
    import json

    value = json.loads(line)
    return value['value'] + len(value['name'])

  with monkeypatch.context() as m:
    m.setattr(main.rust_json, 'sum', python_sum)
    py_result = main.sum(input)

  assert rust_result == py_result
