pub mod observer;
pub mod controller;
pub mod job;
pub use job::JobManager;

#[cfg(test)]
mod tests {
  use super::*;
  use tokio::time::sleep;
  use std::time::Duration;
  #[test]
  fn test_with_cargo() {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    rt.block_on(async move {
      let mut mgr = JobManager::new(&["firefox"], 10.0);
      for _i in 0..10 {
        mgr.watch().await.unwrap();
        sleep(Duration::from_millis(1000)).await;
      }
    });
  }
}
