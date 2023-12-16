## Setup #1
- macbook pro 2019 16
- i9-9880H
- macos 11.6.1

```
taskset -c 0 tarantool tarantool.lua

# epoll (all cores, 16 worker threads)
cargo run -p bench
rps: 1405294
```

## Setup #2
- i9-10920x
- cpupower frequency-set --governor performance
- linux 5.15.6
- mitigations=off

```
taskset -c 0 tarantool tarantool.lua

# epoll (limited to one core)
taskset -c 1 cargo run -p bench
rps: 1461218

# io_uring (limited to one core)
taskset -c 1 cargo run -p bench -- --io_uring
rps: 1448943
```

## Setup #3 (outdated results)
- 5800x
- cpupower frequency-set --governor performance
- linux 5.13.0

```
taskset -c 0 tarantool tarantool.lua

# io_uring (limited to one core)
taskset -c 1 cargo run -p bench -- --io_uring
rps: 1792492

# epoll (limited to one core)
taskset -c 1 cargo run -p bench
rps: 1811333
```

## Setup #4
- i5-12600k
- cpupower frequency-set --governor performance
- linux 5.15.6

```
taskset -c 0 tarantool tarantool.lua

# epoll (limited to one core)
taskset -c 10 cargo run -p bench
rps: 1942238

# io_uring (limited to one core)
taskset -c 10 cargo run -p bench -- --io_uring
rps: 1906886
```
