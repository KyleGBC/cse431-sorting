use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, BatchSize};
use sort_comparison::{insertion_sort, merge_sort};
use rand::{prelude::*, rngs::StdRng};
use std::time::Duration;

fn generate_random_vec(rng: &mut StdRng, len: usize) -> Vec<i32>
{
    let mut v = Vec::with_capacity(len);
    for _ in 0..len
    {  
        v.push(rng.gen::<i32>());
    }
    v
} 

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting Algorithms");
    group.sample_size(20).measurement_time(Duration::from_secs(1));
    let mut seed_rng = rand::thread_rng();
    let mut seed;

    for i in (0..100).map(|n| n * 20)
    {
        // Generate a random seed for this iteration to be shared between each sorting algorithm
        seed = seed_rng.gen::<u64>();

        // Seed an RNG with the shared seed, then run the function on random vectors generated from this seed. 
        // black_box shouldn't be necessary here since the input is random but I used it anyways just in case.
        let mut rng = StdRng::seed_from_u64(seed);
        group.bench_function(BenchmarkId::new("Merge Sort", i), 
        |b| b.iter_batched(
            || generate_random_vec(&mut rng, i),
            |mut data| merge_sort(black_box(&mut data)), 
            BatchSize::SmallInput
            )
        );

        // Reseed the random number generator between sorting alogorithms so make each iteration of the algorithm random, but still the same across algorithms
        let mut rng = StdRng::seed_from_u64(seed);
        group.bench_function(BenchmarkId::new("Insertion Sort", i), 
        |b| b.iter_batched( 
            || generate_random_vec(&mut rng, i),
            |mut data| insertion_sort(black_box(&mut data)),
            BatchSize::SmallInput
            )
        );
    }
    group.finish();

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);