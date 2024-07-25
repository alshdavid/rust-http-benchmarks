# Minimal HTTP server benchmarks

Benchmarks of HTTP servers, the motivation is to figure out why HTTP servers in Rust perform strangely under certain conditions when compared to other languages.

## Instructions

Requirements:
- [Just](https://github.com/casey/just)
- Nodejs
- Go
- Rust

To run the complete benchmark suite and emit an output

```
just benchmark
```

To run an individual server

```
just run rust_std

# Or rebuild on change
just watch rust_std 
just watch rust_std --release
```

