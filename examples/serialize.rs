use benchmark::{collect_data, mean, median, std_deviation};

use benchmark::structs::{ApacheBuilds, GithubEvent, Log, Person};
use serde::Serialize;
use simd_json;
use std::{iter, time::Instant};

#[cfg(feature = "jemallocator")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub const WARMUP: usize = 100;
pub const ROUNDS: usize = 10000;
pub const TRAILS: usize = 100;

fn main() {
    let data_collection = vec![
        (
            "many_fields",
            Box::new(|| smid_json_serialize::<Log>()) as Box<dyn Fn() -> (u64, u64)>,
            Box::new(|| serde_json_serialize::<Log>()) as Box<dyn Fn() -> (u64, u64)>,
        ),
        (
            "nested_json",
            Box::new(|| smid_json_serialize::<GithubEvent>()),
            Box::new(|| serde_json_serialize::<GithubEvent>()),
        ),
        (
            "short_json",
            Box::new(|| smid_json_serialize::<Person>()),
            Box::new(|| serde_json_serialize::<Person>()),
        ),
        (
            "large_vec",
            Box::new(|| smid_json_serialize::<ApacheBuilds>()),
            Box::new(|| serde_json_serialize::<ApacheBuilds>()),
        ),
    ];
    println!("\nSerialization Benchmark");
    report(data_collection);
}

fn report(
    data_collection: Vec<(
        &'static str,
        Box<dyn Fn() -> (u64, u64)>,
        Box<dyn Fn() -> (u64, u64)>,
    )>,
) {
    println!("{}", "-".repeat(150));
    println!("{:<15}|{:^60}     |{:^60}", "", "simd_json", "serde_json");
    println!("{}", "-".repeat(150));
    println!(
        "{:<15}|{:^30}  |{:^30}  |{:^30}  |{:^30}",
        "", "Throughput (MB/s)", "Latency (ns)", "Throughput (MB/s)", "Latency (ns)"
    );
    println!("{}", "-".repeat(150));
    println!(
        "{:<15}|{:^10}|{:^10}|{:^10}|{:^10}|{:^10}|{:^10}|{:^10}|{:^10}|{:^10}|{:^10}|{:^10}|{:^10}",
        "JSON Type",
        "mean",
        "median",
        "std",
        "mean",
        "median",
        "std",
        "mean",
        "median",
        "std",
        "mean",
        "median",
        "std"
    );
    println!("{}", "-".repeat(150));

    for (name, simd_func, serde_func) in &data_collection {
        let simd_throughputs = collect_data(|| simd_func().0, TRAILS);
        let simd_latencies = collect_data(|| simd_func().1, TRAILS);

        let serde_throughputs = collect_data(|| serde_func().0, TRAILS);
        let serde_latencies = collect_data(|| serde_func().1, TRAILS);

        println!(
            "{:^15}|{:^10}|{:^10}|{:^10}|{:^10}|{:^10}|{:^10}|{:^10}|{:^10}|{:^10}|{:^10}|{:^10}|{:^10}",
            name,
            format!("{:.0}", mean(&simd_throughputs)),
            format!("{:.0}", median(&mut simd_throughputs.clone())),
            format!("{:.0}", std_deviation(&simd_throughputs, mean(&simd_throughputs) as f64)),
            format!("{:.0}", mean(&simd_latencies)),
            format!("{:.0}", median(&mut simd_latencies.clone())),
            format!("{:.0}", std_deviation(&simd_latencies, mean(&simd_latencies) as f64)),
            format!("{:.0}", mean(&serde_throughputs)),
            format!("{:.0}", median(&mut serde_throughputs.clone())),
            format!("{:.0}", std_deviation(&serde_throughputs, mean(&serde_throughputs) as f64)),
            format!("{:.0}", mean(&serde_latencies)),
            format!("{:.0}", median(&mut serde_latencies.clone())),
            format!("{:.0}", std_deviation(&serde_latencies, mean(&serde_latencies) as f64))
        );
    }
}

fn smid_json_serialize<T>() -> (u64, u64)
where
    T: Serialize + Default + Clone,
{
    let t = T::default();
    let mut data_entries = iter::repeat(t.clone())
        .take((ROUNDS + WARMUP) as usize)
        .collect::<Vec<T>>();

    let mut r = Ok(Vec::default());
    // Warmups
    for _ in &mut data_entries[..WARMUP] {
        r = simd_json::to_vec(&t);
    }

    // let mut r = Ok(T::default());
    let start = Instant::now();
    for _ in &mut data_entries[WARMUP..] {
        r = simd_json::to_vec(&t);
    }
    let duration = start.elapsed();

    // Ensure that the data is not optimized away
    assert!(r.is_ok());

    // Calculate throughput and latency
    let data_processed_mb: f64 = (r.unwrap().len() * ROUNDS) as f64 / 1024.0 / 1024.0;
    let time_taken_ns = duration.as_nanos() as f64;
    let time_taken_s = time_taken_ns / 1_000_000_000.0;
    let throughput = data_processed_mb / time_taken_s;
    let latency = time_taken_ns / ROUNDS as f64;

    (throughput as u64, latency as u64)
}

fn serde_json_serialize<T>() -> (u64, u64)
where
    T: Serialize + Default + Clone,
{
    let t = T::default();
    let mut data_entries = iter::repeat(t.clone())
        .take((ROUNDS + WARMUP) as usize)
        .collect::<Vec<T>>();

    let mut r = Ok(Vec::default());
    // Warmups
    for _ in &mut data_entries[..WARMUP] {
        r = serde_json::to_vec(&t);
    }

    let start = Instant::now();
    for _ in &data_entries[WARMUP..] {
        r = serde_json::to_vec(&t);
    }
    let duration = start.elapsed();

    // Ensure that the data is not optimized away
    assert!(r.is_ok());

    // Calculate throughput and latency
    let data_processed_mb: f64 = (r.unwrap().len() * ROUNDS) as f64 / 1024.0 / 1024.0;
    let time_taken_ns = duration.as_nanos() as f64;
    let time_taken_s = time_taken_ns / 1_000_000_000.0;
    let throughput = data_processed_mb / time_taken_s;
    let latency = time_taken_ns / ROUNDS as f64;

    (throughput as u64, latency as u64)
}
