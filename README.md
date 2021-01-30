# cmp_error

I was researching *error* of rust, and comparing performance.

## Benchmark Results

This performance test is recursive call program. It is called 1000 times.

- null-void: return Ok(num);
- plainerror: return Err(MyError::new(num));
- std-error: return Err(Box::new(MyError::new(num)));
- thiserror: it use crate thiserror.
- anyhow: it use crate anyhow.
- failure: it use crate failure.

rustc 1.50.0-beta.8 (1cd030396 2021-01-20)
|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| null-void          |   21.078 kc |  277 kib |    0.000 kc |    0 kib |
| plainerror         |   21.188 kc |  278 kib |    0.110 kc |    0 kib |
| thiserror          |   21.272 kc |  278 kib |    0.194 kc |    0 kib |
| anyhow             |   23.379 kc |  283 kib |    2.302 kc |    5 kib |
| std-error          |   30.012 kc |  279 kib |    8.935 kc |    1 kib |
| failure            |   33.505 kc |  462 kib |   12.427 kc |  184 kib |

rustc 1.49.0 (e1884a8e3 2020-12-29)
|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| null-void          |   20.823 kc |  266 kib |    0.000 kc |    0 kib |
| plainerror         |   21.122 kc |  266 kib |    0.299 kc |    0 kib |
| thiserror          |   21.196 kc |  266 kib |    0.374 kc |    0 kib |
| anyhow             |   23.842 kc |  271 kib |    3.019 kc |    5 kib |
| std-error          |   30.038 kc |  267 kib |    9.215 kc |    1 kib |
| failure            |   33.052 kc |  447 kib |   12.230 kc |  181 kib |

rustc 1.42.0 (b8cedc004 2020-03-09)
|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| null-void          |   20.968 kc |  192 kib |    0.000 kc |    0 kib |
| plainerror         |   20.885 kc |  192 kib |   -0.083 kc |    0 kib |
| thiserror          |   21.313 kc |  192 kib |    0.344 kc |    0 kib |
| anyhow             |   26.921 kc |  197 kib |    5.953 kc |    5 kib |
| std-error          |   30.246 kc |  193 kib |    9.277 kc |    1 kib |
| failure            |   38.358 kc |  384 kib |   17.389 kc |  192 kib |

- `kc` is kilo cycles, cycles is cpu clock cycles, lower is better
- `.text` is elf .text section size
- `Δ` delta is the difference from null-void
- bench on intel Q6600 @ 2.40GHz

## What do you think? :octocat:

In application, anyhow is the best, otherwaise, thiserror or plainerror.
The failure is *largest .text* size.
