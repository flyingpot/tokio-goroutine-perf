# tokio-goroutine-perf

Some of the code are from [here](https://www.reddit.com/r/rust/comments/lg0a7b/benchmarking_tokio_tasks_and_goroutines/).

Rust results:

```shell
cargo run --release
```

Go result:

```shell
go run goroutine.go
```

However, this benchmark cannot represent I/O performance result. Because /dev/urandom read and /dev/null write cannot trigger any I/O systemcall.

The Rust thread is slow because the cost of thread creation. And why block tokio is faster than unblock one is that this read and write is not blocking, so unblock tokio will make task switch more.
