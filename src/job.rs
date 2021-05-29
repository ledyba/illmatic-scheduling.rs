use std::collections::HashMap;
use std::mem::swap;

use tokio::io;
use tokio::sync::mpsc;

use crate::controller::PidController;
use crate::observer::observe_nonvoluntary_ctxt_switches;
use crate::observer::collect_processes;

pub struct JobManager {
  targets: Vec<String>,
  pid: PidController,
  switches: HashMap<u32, u32>
}

impl JobManager {
  pub fn new<T>(targets:&[T], set_point: f32) -> Self
  where T: AsRef<str> {
    Self {
      targets: targets.iter().map(|s| s.as_ref().to_string()).collect(),
      pid: PidController::new(set_point),
      switches: HashMap::new(),
    }
  }
  pub async fn watch(&mut self) -> io::Result<u32> {
    let processes = collect_processes(&self.targets).await?;
    
    let (tx, mut rx) = mpsc::channel(100);
    for (_, pid) in processes.iter() {
      let pid = *pid;
      let tx = tx.clone();
      tokio::spawn(async move {
        let result = observe_nonvoluntary_ctxt_switches(pid).await.map(|switches| (pid, switches));
        tx.send(result).await.expect("Failed to send MPSC messenger");
      });
    }
    let mut current_switches:HashMap<u32, u32> = HashMap::new();
    let mut cnt: usize = 0;
    while let Some(result) = rx.recv().await {
      if let Ok((pid, switch)) = result {
        current_switches.insert(pid, switch);
      }
      cnt += 1;
      if cnt >= processes.len() {
        break;
      }
    }
    let mut delta: u32 = 0;
    let mut cnt: u32 = 0;
    for (pid, switch) in &current_switches {
      if let Some(old) = self.switches.get(pid) {
        delta += *switch - *old;
        cnt += 1;
      }
    }
    let delta = if cnt == 0 {
      0.0
    } else {
      delta as f32 / cnt as f32
    };
    swap(&mut self.switches, &mut current_switches);
    let out = self.pid.next(delta as f32);
    Ok(out as u32)
  }
}
