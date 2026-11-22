function constant(n: number): number {
  const steps = 1; // @step constant
  return steps;
}

function logarithmic(n: number): number {
  let steps = 0;
  for (let i = 1; i <= n; i *= 2) {
    steps++; // @step log
  }
  return steps;
}

function linear(n: number): number {
  let steps = 0;
  for (let i = 0; i < n; i++) {
    steps++; // @step linear
  }
  return steps;
}

function linearithmic(n: number): number {
  let steps = 0;
  for (let i = 0; i < n; i++) {
    for (let j = 1; j <= n; j *= 2) {
      steps++; // @step nlogn
    }
  }
  return steps;
}

function quadratic(n: number): number {
  let steps = 0;
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < n; j++) {
      steps++; // @step quadratic
    }
  }
  return steps;
}

function exponential(n: number): number {
  let steps = 0;
  for (let subset = 0; subset < 2 ** n; subset++) {
    steps++; // @step exponential
  }
  return steps;
}

function countSteps(n: number): number[] {
  return [
    constant(n),
    logarithmic(n),
    linear(n),
    linearithmic(n),
    quadratic(n),
    exponential(n),
  ];
}
