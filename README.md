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

rustc 1.56.1 (59eed8a2a 2021-11-01):

|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| thiserror          |   21.446 kc |  295 kib |   -0.427 kc |    0 kib |
| plainerror         |   21.704 kc |  295 kib |   -0.168 kc |    0 kib |
| null-void          |   21.873 kc |  294 kib |    0.000 kc |    0 kib |
| anyhow             |   22.139 kc |  300 kib |    0.266 kc |    5 kib |
| std-error          |   30.299 kc |  296 kib |    8.426 kc |    1 kib |
| failure            |   34.347 kc |  501 kib |   12.474 kc |  206 kib |

rustc 1.53.0 (53cb7b09b 2021-06-17):

|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| null-void          |   21.317 kc |  262 kib |    0.000 kc |    0 kib |
| plainerror         |   21.359 kc |  263 kib |    0.042 kc |    0 kib |
| thiserror          |   21.478 kc |  263 kib |    0.160 kc |    0 kib |
| anyhow             |   23.576 kc |  267 kib |    2.258 kc |    5 kib |
| std-error          |   30.058 kc |  264 kib |    8.740 kc |    1 kib |
| failure            |   33.265 kc |  453 kib |   11.948 kc |  190 kib |

rustc 1.52.0 (88f19c6da 2021-05-03):

|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| plainerror         |   20.822 kc |  265 kib |   -0.135 kc |    0 kib |
| null-void          |   20.957 kc |  265 kib |    0.000 kc |    0 kib |
| thiserror          |   21.095 kc |  265 kib |    0.138 kc |    0 kib |
| anyhow             |   23.668 kc |  270 kib |    2.712 kc |    5 kib |
| std-error          |   29.673 kc |  266 kib |    8.717 kc |    1 kib |
| failure            |   32.121 kc |  455 kib |   11.165 kc |  190 kib |


rustc 1.51.0 (2fd73fabe 2021-03-23):

|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| thiserror          |   21.127 kc |  276 kib |   -0.038 kc |    0 kib |
| null-void          |   21.165 kc |  276 kib |    0.000 kc |    0 kib |
| plainerror         |   21.312 kc |  276 kib |    0.147 kc |    0 kib |
| anyhow             |   23.188 kc |  281 kib |    2.023 kc |    5 kib |
| std-error          |   29.566 kc |  277 kib |    8.401 kc |    1 kib |
| failure            |   34.877 kc |  459 kib |   13.712 kc |  182 kib |

rustc 1.50.0 (cb75ad5db 2021-02-10):
|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| null-void          |   21.241 kc |  276 kib |    0.000 kc |    0 kib |
| thiserror          |   21.267 kc |  277 kib |    0.026 kc |    0 kib |
| plainerror         |   21.306 kc |  277 kib |    0.066 kc |    0 kib |
| anyhow             |   23.124 kc |  282 kib |    1.883 kc |    5 kib |
| std-error          |   29.110 kc |  278 kib |    7.869 kc |    1 kib |
| failure            |   37.951 kc |  456 kib |   16.711 kc |  179 kib |

rustc 1.49.0 (e1884a8e3 2020-12-29)
|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| null-void          |   21.057 kc |  266 kib |    0.000 kc |    0 kib |
| thiserror          |   21.218 kc |  266 kib |    0.162 kc |    0 kib |
| plainerror         |   21.469 kc |  266 kib |    0.412 kc |    0 kib |
| anyhow             |   23.615 kc |  271 kib |    2.558 kc |    5 kib |
| std-error          |   29.673 kc |  267 kib |    8.616 kc |    1 kib |
| failure            |   34.801 kc |  447 kib |   13.745 kc |  181 kib |

rustc 1.42.0 (b8cedc004 2020-03-09)
|       `name`       |   `bench`   | `.text`  |  `Δ bench`  | `Δ .text` |
|:-------------------|------------:|---------:|------------:|---------:|
| null-void          |   20.973 kc |  192 kib |    0.000 kc |    0 kib |
| thiserror          |   21.057 kc |  192 kib |    0.084 kc |    0 kib |
| plainerror         |   21.238 kc |  192 kib |    0.264 kc |    0 kib |
| anyhow             |   26.792 kc |  197 kib |    5.818 kc |    5 kib |
| std-error          |   29.818 kc |  193 kib |    8.845 kc |    1 kib |
| failure            |   34.782 kc |  379 kib |   13.808 kc |  187 kib |

- `kc` is kilo cycles, cycles is cpu clock cycles, lower is better
- `.text` is elf .text section size
- `Δ` delta is the difference from null-void
- bench on intel Q6600 @ 2.40GHz

## What do you think? :octocat:

In application, anyhow is the best, otherwaise, thiserror or plainerror.
The failure is *largest .text* size.
