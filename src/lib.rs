pub mod kernel;

pub mod pid;

pub mod job;
pub use job::Scheduler;
pub use job::Builder;

#[cfg(test)]
mod tests {
  use super::*;
  use tokio::time::sleep;
  use std::time::Duration;
  #[test]
  fn test_with_cargo() {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    rt.block_on(async move {
      let mut sch = Builder::new().add_target_process("firefox").set_point(10.0).build();
      for _i in 0..10 {
        sch.watch().await.unwrap();
        sleep(Duration::from_millis(1000)).await;
      }
    });
  }
}
