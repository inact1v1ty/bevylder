# 🌳 Bevylder

[![License: MIT/Apache](https://img.shields.io/badge/License-MIT%20or%20Apache2-blue.svg)](https://opensource.org/licenses/MIT)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-v0.7-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

Voxels brought to bevy.

***

### WARNING!
This crate is in a Proof of Concept stage of development and is not ready to be used in projects!

But keep an eye on it 🙂 

### To run

Two ducks:
```sh
cargo run --release --features="bevy/bevy_winit","bevy/dynamic" --example rubberduck
```

2N + 1 x 2N + 1 ducks stresstest:
```sh
cargo run --release --features="bevy/bevy_winit","bevy/dynamic" --example rubberduck -- --stress <N>
```

***

### License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
