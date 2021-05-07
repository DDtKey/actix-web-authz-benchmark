### Minimal examples of using [`casbin-rs`] and [`actix-web-grants`] and performance comparisons

The [`wrk`] tool was used for benchmarks (all commands used can be found in `bench.sh`)

##### How to run benchmarks
1. To run you need [`cargo`] and [`wrk`] installed
2. Clone the `actix-web-authz-benchmark` repository
3. Run the shell script `bench.sh` in the root of project directory
```shell
sh bench.sh
```

#### Table of benchmark results

<table>
    <tr>
      <td rowspan="2">Benchmark</td>
      <td colspan="2"><b>actix-web-grants</b></td>
      <td colspan="2"><b>casbin-rs</b></td>
    </tr>
    <tr>
      <td>Latency</td>
      <td>Req/Sec</td>
      <td>Latency</td>
      <td>Req/Sec</td>
    </tr>
    <tr>
      <td>Allowed Endpoint</td>
      <td>4.41 ms </td>
      <td>22.69k </td>
      <td>6.18 ms </td>
      <td>16.27k</td>
    </tr>
    <tr>
      <td>Denied Endpoint</td>
      <td>4.94 ms</td>
      <td>20.23k</td>
      <td>6.70 ms</td>
      <td>14.98k</td>
    </tr>
</table>

> rustc: v1.52.0 (stable); CPU: 2,6 GHz 6-Core Intel Core i7; RAM: 16 GB

[`actix-web-grants`]: https://github.com/DDtKey/actix-web-grants
[`casbin-rs`]: https://github.com/casbin-rs/actix-casbin-auth
[`wrk`]: https://github.com/wg/wrk
[`cargo`]: https://doc.rust-lang.org/stable/cargo/getting-started/installation.html
