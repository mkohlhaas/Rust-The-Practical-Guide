fn sort_benchmark(c: &mut Criterion) {
  ...
   c.bench_function("Sorting Algorithm", |b| {
        b.iter(|| sort_algo_2(&mut numbers)) // changed to algo_2
    });
}