#include <stdio.h>
#include <string.h>

int solve(char *line, int *solution);

int main() {
  char line[100]; // <1>
  int solution;

  while (1) {
    printf("> ");
    if (fgets(line, 100, stdin) == NULL) { // <2>
      return 0;
    }

    if (solve(line, &solution)) {
      continue;
    }

    printf("%d\n", solution);
  }

  return 0;
}

int solve(char *line, int *solution) { // <3>
  int num1, num2;
  char operator;

  int values_read = sscanf(line, "%d %d %c", &num1, &num2, &operator); // <4>
  if (values_read != 3) {
    return 1;
  }

  switch (operator) {
  case '+':
    *solution = num1 + num2; // <5>
    return 0;
  case '-':
    *solution = num1 - num2;
    return 0;
  case '*':
    *solution = num1 * num2;
    return 0;
  case '/':
    *solution = num1 / num2;
    return 0;
  }

  return 1;
}