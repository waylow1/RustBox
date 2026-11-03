# RustBox

RustBox is an experimental Linux container runtime written in Rust.

## Current system milestone

- Launches a separate manager process for each container.
- Creates UTS, PID, and mount namespaces and runs the configured command as PID 1.
- Uses `chroot`, makes mount propagation private, and mounts `/proc` inside the container.
- Applies cgroups v2 limits of 2 CPU cores and 2 GiB of memory to the command process.

The demo expects an Alpine root filesystem at `./alpine`, including `/bin/sh` and `/proc`. Run it on Linux with permission to create namespaces and manage `/sys/fs/cgroup`. The cgroup v2 `cpu` and `memory` controllers must be available; RustBox fails rather than silently running without configured limits.

CPU limits are expressed as a number of cores; memory limits are expressed in bytes.
