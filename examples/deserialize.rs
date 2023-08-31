use benchmark::{collect_data, mean, median, std_deviation};

use benchmark::structs::{ApacheBuilds, GithubEvent, Log, Person};
use serde::Deserialize;
use simd_json;
use std::{env, fs, iter, time::Instant};

#[cfg(feature = "jemallocator")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub const WARMUP: usize = 100;
pub const ROUNDS: usize = 10000;
pub const TRAILS: usize = 200;

fn main() {
    let mut opts = getopts::Options::new();
    opts.optflag("s", "struct", "benchmark struct deserialization");
    opts.optflag("v", "value", "benchmark value deserialization");
    let args: Vec<String> = env::args().collect();
    let matches = opts.parse(&args[1..]).unwrap();

    if matches.opt_present("struct") {
        let data_collection = vec![
            (
                "many_fields",
                Box::new(|name: &_| smid_json_to_struct::<Log>(name))
                    as Box<dyn Fn(&str) -> (u64, u64)>,
                Box::new(|name: &_| serde_json_deserialize::<Log>(name))
                    as Box<dyn Fn(&str) -> (u64, u64)>,
            ),
            (
                "nested_json",
                Box::new(|name| smid_json_to_struct::<GithubEvent>(name)),
                Box::new(|name| serde_json_deserialize::<GithubEvent>(name)),
            ),
            (
                "small_json",
                Box::new(|name| smid_json_to_struct::<Person>(name)),
                Box::new(|name| serde_json_deserialize::<Person>(name)),
            ),
            (
                "large_vec",
                Box::new(|name| smid_json_to_struct::<ApacheBuilds>(name)),
                Box::new(|name| serde_json_deserialize::<ApacheBuilds>(name)),
            ),
        ];
        println!("Deserialization Benchmark (Struct)");
        report(data_collection);
    } else if matches.opt_present("value") {
        let data_collection = vec![
            (
                "many_fields",
                Box::new(|name: &_| smid_json_to_value(name)) as Box<dyn Fn(&str) -> (u64, u64)>,
                Box::new(|name: &_| serde_json_deserialize::<serde_json::Value>(name))
                    as Box<dyn Fn(&str) -> (u64, u64)>,
            ),
            (
                "nested_json",
                Box::new(|name| smid_json_to_value(name)),
                Box::new(|name| serde_json_deserialize::<serde_json::Value>(name)),
            ),
            (
                "small_json",
                Box::new(|name| smid_json_to_value(name)),
                Box::new(|name| serde_json_deserialize::<serde_json::Value>(name)),
            ),
            (
                "large_vec",
                Box::new(|name| smid_json_to_value(name)),
                Box::new(|name| serde_json_deserialize::<serde_json::Value>(name)),
            ),
        ];
        println!("Deserialization Benchmark (Value)");
        report(data_collection);
    } else {
        println!("Please specify either -s or -v");
    }
}

fn report(
    data_collection: Vec<(
        &'static str,
        Box<dyn Fn(&str) -> (u64, u64)>,
        Box<dyn Fn(&str) -> (u64, u64)>,
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
        let simd_throughputs = collect_data(|| simd_func(name).0, TRAILS);
        let simd_latencies = collect_data(|| simd_func(name).1, TRAILS);

        let serde_throughputs = collect_data(|| serde_func(name).0, TRAILS);
        let serde_latencies = collect_data(|| serde_func(name).1, TRAILS);

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

fn smid_json_to_value(json_file: &str) -> (u64, u64) {
    let filepath = format!("data/{}.json", json_file);
    let json_bytes = fs::read(&filepath).unwrap();
    let single_json_size = json_bytes.len() as f64;

    let mut data_entries = iter::repeat(json_bytes)
        .take((ROUNDS + WARMUP) as usize)
        .collect::<Vec<Vec<u8>>>();

    // Warmups
    for data_entry in &mut data_entries[..WARMUP] {
        let _ = simd_json::to_borrowed_value(data_entry);
    }

    let mut r = Ok(simd_json::value::borrowed::Value::default());
    let start = Instant::now();
    for data_entry in &mut data_entries[WARMUP..] {
        r = simd_json::to_borrowed_value(data_entry);
    }
    let duration = start.elapsed();

    // Ensure that the data is not optimized away
    assert!(r.is_ok());

    // Calculate throughput and latency
    let data_processed_mb: f64 = single_json_size * ROUNDS as f64 / 1024.0 / 1024.0;
    let time_taken_ns = duration.as_nanos() as f64;
    let time_taken_s = time_taken_ns / 1_000_000_000.0;
    let throughput = data_processed_mb / time_taken_s;
    let latency = time_taken_ns / ROUNDS as f64;

    (throughput as u64, latency as u64)
}

fn smid_json_to_struct<T>(json_file: &str) -> (u64, u64)
where
    T: for<'a> Deserialize<'a> + Default,
{
    let filepath = format!("data/{}.json", json_file);
    let json_bytes = fs::read(&filepath).unwrap();
    let single_json_size = json_bytes.len() as f64;

    let mut data_entries = iter::repeat(json_bytes)
        .take((ROUNDS + WARMUP) as usize)
        .collect::<Vec<Vec<u8>>>();
    let mut r = Ok(T::default());
    // Warmups
    for data_entry in &mut data_entries[..WARMUP] {
        r = simd_json::from_slice(data_entry);
    }

    let start = Instant::now();
    for data_entry in &mut data_entries[WARMUP..] {
        r = simd_json::from_slice(data_entry);
    }
    let duration = start.elapsed();

    // Ensure that the data is not optimized away
    assert!(r.is_ok());

    // Calculate throughput and latency
    let data_processed_mb: f64 = single_json_size * ROUNDS as f64 / 1024.0 / 1024.0;
    let time_taken_ns = duration.as_nanos() as f64;
    let time_taken_s = time_taken_ns / 1_000_000_000.0;
    let throughput = data_processed_mb / time_taken_s;
    let latency = time_taken_ns / ROUNDS as f64;

    (throughput as u64, latency as u64)
}

fn serde_json_deserialize<T>(json_file: &str) -> (u64, u64)
where
    T: for<'a> Deserialize<'a> + Default,
{
    let filepath = format!("data/{}.json", json_file);
    let json_bytes = fs::read(&filepath).unwrap();
    let single_json_size = json_bytes.len() as f64;

    let mut data_entries = iter::repeat(json_bytes)
        .take((ROUNDS + WARMUP) as usize)
        .collect::<Vec<Vec<u8>>>();

    let mut r = Ok(T::default());
    // Warmups
    for data_entry in &mut data_entries[..WARMUP] {
        r = serde_json::from_slice(data_entry);
    }

    let start = Instant::now();
    for data_entry in &data_entries[WARMUP..] {
        r = serde_json::from_slice(data_entry);
    }
    let duration = start.elapsed();

    // Ensure that the data is not optimized away
    assert!(r.is_ok());

    // Calculate throughput and latency
    let data_processed_mb: f64 = single_json_size * ROUNDS as f64 / 1024.0 / 1024.0;
    let time_taken_ns = duration.as_nanos() as f64;
    let time_taken_s = time_taken_ns / 1_000_000_000.0;
    let throughput = data_processed_mb / time_taken_s;
    let latency = time_taken_ns / ROUNDS as f64;

    (throughput as u64, latency as u64)
}
