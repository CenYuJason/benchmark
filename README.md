# Benchmarking simd_json and serde_json
This repository contains a simple benchmark that compares the performance of simd_json and serde_json on deserializing and serializing JSON data.

# Usage
To run the benchmark, consider the following three command lines:

1. cargo run --release --example deserialize --features mimalloc -- -s
2. cargo run --release --example deserialize --features mimalloc -- -v
3. cargo run --release --example serialize --features mimalloc

The first command line will benchmark the deserialization of JSON data into a struct using the mimalloc memory allocator. The second command line will benchmark the deserialization of JSON data into Value using the mimalloc memory allocator. The third command line will benchmark the serialization of a struct into JSON data using the mimalloc memory allocator.

# Example output
<img width="685" alt="image" src="https://github.com/CenYuJason/benchmark/assets/96949397/afdfbd33-cc91-4158-a579-8b60f5130ee8">

