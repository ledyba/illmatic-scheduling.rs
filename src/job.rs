use std::collections::HashMap;
use std::mem::swap;

use tokio::io;
use tokio::sync::mpsc;

use crate::pid::PidController;
use crate::kernel::observe_nonvoluntary_ctxt_switches;
use crate::kernel::collect_processes;

pub struct Builder {
  targets: Vec<String>,
  p_gain: f32,
  i_gain: f32,
  d_gain: f32,
  set_point: f32,
}

impl Builder {
  pub fn new() -> Self {
    Self {
      targets: Default::default(),
      p_gain: 0.5,
      i_gain: 0.1,
      d_gain: 0.01,
      set_point: 10.0,
    }
  }
  pub fn add_target_process<T>(&mut self, process_name: T) -> &mut Self
    where T: AsRef<str> 
  {
    self.targets.push(process_name.as_ref().to_string());
    self
  }
  pub fn p_gain(&mut self, p_gain: f32) -> &mut Self {
    self.p_gain = p_gain;
    self
  }
  pub fn i_gain(&mut self, i_gain: f32) -> &mut Self {
    self.i_gain = i_gain;
    self
  }
  pub fn d_gain(&mut self, d_gain: f32) -> &mut Self {
    self.d_gain = d_gain;
    self
  }
  pub fn set_point(&mut self, set_point: f32) -> &mut Self {
    self.set_point = set_point;
    self
  }
  pub fn build(&mut self) -> Scheduler {
    Scheduler {
      targets: self.targets.clone(),
      pid: PidController::new(self.p_gain, self.i_gain, self.d_gain, self.set_point),
      switches: Default::default(),
    }
  }
}

pub struct Scheduler {
  targets: Vec<String>,
  pid: PidController,
  switches: HashMap<u32, u32>
}

impl Scheduler {
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
    for _i in 0..processes.len() {
      if let Some(result) = rx.recv().await {
        if let Ok((pid, switch)) = result {
          current_switches.insert(pid, switch);
        }
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
  pub fn pid(&self) -> &PidController {
    &self.pid
  }
  pub fn pid_mut(&mut self) -> &mut PidController {
    &mut self.pid
  }
  pub fn targets(&self) -> &[String] {
    &self.targets
  }
}
