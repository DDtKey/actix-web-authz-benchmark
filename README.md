### Minimal examples of using [`casbin-rs`] and [`actix-web-grants`] and performance comparisons

Benchmarks are written using [`criterion`] and are located in this repository.

#### Table of benchmark results

<table>
    <tr>
      <td rowspan="2">Benchmark</td>
      <td colspan="3"><b>actix-web-grants</b></td>
      <td colspan="3"><b>casbin-rs</b></td>
    </tr>
    <tr>
      <td>Min</td>
      <td>Mean</td>
      <td>Max</td>
      <td>Min</td>
      <td>Mean</td>
      <td>Max</td>
    </tr>
    <tr>
      <td>Allowed Endpoint</td>
      <td>12.157 us</td>
      <td>12.433 us</td>
      <td>12.736 us</td>
      <td>354.85 us</td>
      <td>363.26 us</td>
      <td>373.44 us</td>
    </tr>
    <tr>
      <td>Denied Endpoint</td>
      <td>11.165 us</td>
      <td>11.239 us</td>
      <td>11.317 us</td>
      <td>304.16 us</td>
      <td>324.52 us</td>
      <td>350.22 us</td>
    </tr>
</table>

> CPU: 2,6 GHz 6-Core Intel Core i7; RAM: 16 GB

[`actix-web-grants`]: https://github.com/DDtKey/actix-web-grants
[`casbin-rs`]: https://github.com/casbin/casbin-rs
[`criterion`]: https://github.com/bheisler/criterion.rs
