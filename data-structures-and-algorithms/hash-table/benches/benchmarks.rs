use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use hash_table::{HashTable, LinkedHashTable};

fn bench_operations(c: &mut Criterion) {
    let sizes = [100, 1000, 10000, 100000];

    // Benchmark insert
    let mut group = c.benchmark_group("insert");
    for size in sizes {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter_with_setup(
                || LinkedHashTable::new(size), // Create new table for each iteration
                |mut table| {
                    let key = format!("key{}", size / 2); // Use consistent key
                    table.insert(key, size)
                },
            );
        });
    }
    group.finish();

    // Benchmark get
    let mut group = c.benchmark_group("get");
    for size in sizes {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut table = LinkedHashTable::new(size);
            // Pre-fill table to half capacity
            for i in 0..size / 2 {
                table.insert(format!("key{}", i), i);
            }
            let key = format!("key{}", size / 4); // Get from middle of filled portion
            b.iter(|| table.get(&key));
        });
    }
    group.finish();

    // Benchmark get_first
    let mut group = c.benchmark_group("get_first");
    for size in sizes {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut table = LinkedHashTable::new(size);
            // Pre-fill table to half capacity
            for i in 0..size / 2 {
                table.insert(format!("key{}", i), i);
            }
            b.iter(|| table.get_first());
        });
    }
    group.finish();

    // Benchmark get_last
    let mut group = c.benchmark_group("get_last");
    for size in sizes {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut table = LinkedHashTable::new(size);
            // Pre-fill table to half capacity
            for i in 0..size / 2 {
                table.insert(format!("key{}", i), i);
            }
            b.iter(|| table.get_last());
        });
    }
    group.finish();

    // Benchmark remove
    let mut group = c.benchmark_group("remove");
    for size in sizes {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter_with_setup(
                || {
                    let mut table = LinkedHashTable::new(size);
                    // Pre-fill table to half capacity
                    for i in 0..size / 2 {
                        table.insert(format!("key{}", i), i);
                    }
                    table
                },
                |mut table| {
                    let key = format!("key{}", size / 4); // Remove from middle of filled portion
                    table.remove(&key)
                },
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_operations);
criterion_main!(benches);
