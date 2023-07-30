A Github-like, Rust-based heatmap generator for non-git-based local & remote directories

# Origin directory support
Anything supported by `rclone`!

# Usage
## Manual invocation
TODO: CLI interface here

## Serving mode
Want to have this done automatically for you?
- Spin up infra
    - Lambda to run on a cron schedule
    - S3 bucket for serving

# Implementation
## Notes
Operating systems don't track the history of modifications. This is solved by
Git (among many other things). We want to replicate the subset of this behaviour
that tracks history of modifications.

For remote directories the remote directory is cloned upon each invocation in the interest of
implementation simplicity & avoiding storage costs. This might not make sense
for some usecases. Tracked for future development (see [TDL](#To Do List)).

## Flow
- `rclone` remote config setup
- `rclone` to quickly support [many](https://rclone.org/overview/) remote
  storage solutions & normalize to a singular local interface
    - [Pre-compiled](https://docs.rs/librclone/latest/librclone/) into the Rust binary
        - TODO: how does this affect cold start time?
- Rust program
    - tracks state via human readable JSON
        - Stored locally or in S3
    - generates heatmap
- Publishes to S3

# Project details
## Goals
- Track only enough data to generate a heatmap
- Use Rust

## Non-goals
- Inherit VCS-level complexity
- Use Go (even tho the _right_ choice w.r.t rclone being in Go)
- Use Git to track state
    - More optimal w.r.t not re-inventing the wheel, sub-optimal for learning
      new things!

# To Do List
- [] Support for various outputs aside S3
- [] Support `rclone sync` with persistant replicated storage

# Rough Notes
- can we do some iterated random sampling from a file to replace chucking
  everything into a hash (which is not robust to file changes)
