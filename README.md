# Fractal Trees

This Rust program generates fractal trees

![Example image](out.png)

## Build

Build with cargo:

```bash
cargo build --release
```

## Arguments

```text
Options:
      --path <PATH>                    Path of output image [default: out.png]
  -i, --iterations <ITERATIONS>        Number of iterations [default: 15]
  -a, --angle <ANGLE>                  Angle offset of each branch [default: 22]
  -b, --branch-length <BRANCH_LENGTH>  Init branch length [default: 300]
  -r, --ratio <RATIO>                  Ratio of branch length [default: 0.8]
      --resx <RESX>                    Image resolution x [default: 2048]
      --resy <RESY>                    Image resolution y [default: 2048]
  -p, --progress                       Print progress of generation
  -h, --help                           Print help
  -V, --version                        Print version
```

## Changelog

### 1.0.1
- Fixed slow performance when progress mode is enabled

### 1.0.0
- First version

## Future plans
- Non symmetrical branches
- Settable number of branches
- Automatic resolution
- Branch thickness
- Colors
