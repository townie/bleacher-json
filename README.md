# General benchmarking for Rust Json libraries

<table>
  <tr>
Disclaimer
  </tr>
  <tr>
This is a learning exercise and has room for improvements, suggestions and PRs are welcome
  </tr>
</table>



Using a nightly rustc compiler, ran `cargo bench`

```
running 4 tests
test tests::benchmark_sedre_json         ... bench:         359.91 ns/iter (+/- 32.71)
test tests::benchmark_simd_json_borrowed ... bench:         798.63 ns/iter (+/- 497.40)
test tests::benchmark_simd_json_owned    ... bench:       1,155.71 ns/iter (+/- 67.33)
test tests::benchmark_simdjson_rust      ... bench:          88.75 ns/iter (+/- 5.72)
```
