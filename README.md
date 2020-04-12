# Calculating Pi with collisions

This repository contains a simulation between two elastically colliding blocks
with different masses. The simulation
demonstrates how the amount of collisions between a block with mass $`1`$ and
another block with mass $`100^{n - 1}`$, where $`n`$ is amount of digits 
to compute, equals $`\Pi`$. It's written in Rust using `ggez`. To understand why
this occurs, refer to following video: 
[Why do colliding blocks compute pi?](https://www.youtube.com/watch?v=jsYwFizhncE)
You can also read this article: 
[Throwing pi at a wall](https://arxiv.org/pdf/1901.06260.pdf).

![Demonstration](./DEMO.png "Demonstration")

# How to run?

First install `Rustup`: [How to install?](https://www.rust-lang.org/tools/install)
With the Rust compiler installed, run the following commands (Note: running in
release mode increases performance by tenfolds):

```shell
git clone https://gitlab.com/knarkzel/pi-with-collisions
cd pi-with-collisions/
cargo run --release
```

# Value table to calculate with minimum required timesteps

These variables must be modified in `src/main.rs` to compute $`n`$ digits of Pi.

| Digits |  Timesteps |
|--------|------------|
|    1.0 |        1.0 |
|    2.0 |        1.0 |
|    3.0 |        3.0 |
|    4.0 |       15.0 |
|    5.0 |      154.0 |
|    6.0 |     1554.0 |
|    7.0 |    15598.0 |
|    8.0 |   156165.0 |
|    9.0 |  1562228.0 |
|   10.0 | 15630000.0 |
