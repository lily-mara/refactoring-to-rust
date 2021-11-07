use pyo3::prelude::*;
use pyo3::types::PyDict;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use criterion::{
  black_box, criterion_group, criterion_main, Bencher, Criterion,
  Throughput,
};
use std::{fmt::Write as _, iter, time::Duration};

fn bench_py(input: &str, imports: &[&str], code: &str, b: &mut Bencher) {
  pyo3::prepare_freethreaded_python();

  Python::with_gil(|py| {
    let locals = PyDict::new(py);

    for i in imports {
      locals.set_item(i, py.import(i).unwrap()).unwrap();
    }

    locals.set_item("INPUT", input).unwrap();

    b.iter(|| black_box(py.run(code, None, Some(&locals)).unwrap()));
  });
}

fn json_input(count: u64) -> String {
  let mut s = String::new();
  let mut rng = thread_rng();

  for _ in 0..count {
    let value: u16 = rng.gen();
    let name: String = iter::repeat(())
      .map(|()| rng.sample(Alphanumeric))
      .map(char::from)
      .take(7)
      .collect();

    writeln!(&mut s, r#"{{ "name": "{}", "value": {} }}"#, name, value)
      .unwrap();
  }

  s
}

fn json_group(c: &mut Criterion) {
  let count = 1;

  let input = json_input(count);

  let mut group = c.benchmark_group("json");
  group.throughput(Throughput::Elements(count));

  group.bench_function("python-json", |b| {
    bench_py(
      &input,
      &["json"],
      "s = 0
for line in INPUT.splitlines():
  value = json.loads(line)
  s += value['value']
  s += len(value['name'])",
      b,
    )
  });

  group.bench_function("PyO3-serde-json", |b| {
    bench_py(
      &input,
      &["rust_json"],
      "s = 0
for line in INPUT.splitlines():
  s += rust_json.sum(line)",
      b,
    )
  });

  drop(group);

  c.bench_function("python-raw", |b| {
    bench_py(&input, &["json"], "1 + 1", b)
  });
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = json_group
);
criterion_main!(benches);
