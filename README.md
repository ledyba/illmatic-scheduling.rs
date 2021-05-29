# Illmatic Scheduling: A scheduling library avoiding to dirturb someone.

## How to use?

```rust
  let rt = tokio::runtime::Builder::new_multi_thread().enable_time().build().unwrap();
  rt.block_on(async move {
    let mut mgr = illmatic_scheduling::JobManager::new(&["firefox"], 10.0);
    for _i in 0..10 {
      if let Ok(count) = mgr.watch().await {
        // You have a `count` times to do something.
      }
      sleep(Duration::from_millis(1000)).await;
    }
  });
```

# License

AGPL v3 or later
