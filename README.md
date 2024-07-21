dp32g030
========
[![crates.io](https://img.shields.io/crates/v/dp32g030.svg?label=dp32g030)](https://crates.io/crates/dp32g030)

Peripheral Access Crate for the DP32G030 microcontroller. The API is generated using [svd2rust](https://github.com/rust-embedded/svd2rust).

Usage
-----
In your `Cargo.toml` (features are optional):
```toml
[dependencies]
dp32g030 = { version = "1.0.0", features = ["critical-section", "defmt", "rt"] }
```
In your code:
```rust
use dp32g030;

let peripherals = dp32g030::Peripherals::take().unwrap();
```
Refer to the [svd2rust documentation](https://docs.rs/svd2rust) for further usage.

Credits
-------
- [amnemonic](https://github.com/amnemonic) for the original [svd](https://github.com/amnemonic/Quansheng_UV-K5_Firmware/blob/0255bca35f0f4d95bd67c3c4406af798e8a8a2df/hardware/DP32G030/DP32G030.svd) mine is extended from
- [itsme](https://github.com/itsme2417/) for assisting in enriching the svd with data from the datasheet

License
-------
This project is subject to the terms of the [Mozilla Public License, v. 2.0](./LICENSE).