
## Setup #1
- i9-10920x
- cpupower frequency-set --governor performance
- linux 5.15.6
- mitigations=off

```
taskset -c 0 tarantool tarantool.lua

# io_uring (limited to one core)
taskset -c 8 cargo run -p bench -- --io_uring
rps: 1262784
```

## Setup #2
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
