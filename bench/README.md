### Setup
- i9-10920x
- cpupower frequency-set --governor performance
- linux 5.15.6
- mitigations=off

```
taskset -c 0 tarantool tarantool.lua
taskset -c 8 cargo run -p bench -- --io_uring
```

### Result
```
rps: 1262784
```

