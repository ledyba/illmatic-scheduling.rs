use tokio::fs;
use tokio::io;
use regex::Regex;

pub async fn collect_processes<T>(process_names: &[T]) -> io::Result<Vec<(String, u32)>>
where
  T: AsRef<str>,
{
  let mut result: Vec<(String, u32)> = Vec::new();

  let mut entries = fs::read_dir("/proc").await?;
  while let Some(entry) = entries.next_entry().await? {
    let path = entry.path();
    let dirname = if let Some(dirname) = path.strip_prefix("/proc").unwrap().to_str() {
      dirname
    } else {
      continue;
    };
    let pid = if let Ok(pid) = dirname.parse::<u32>() {
      pid
    } else {
      continue;
    };
    let real_path = fs::read_link(&entry.path().join("exe")).await;
    if let Ok(real_path) = real_path {
      if let Some(real_path) = real_path.to_str() {
        for name in process_names.iter() {
          if real_path.contains(name.as_ref()) {
            result.push((name.as_ref().to_string(), pid));
          }
        }
      }
    }
  }
  io::Result::Ok(result)
}

static NV_CTXT_PAT: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
  Regex::new(r"nonvoluntary_ctxt_switches:	(\d+)").unwrap()
});

pub async fn observe_nonvoluntary_ctxt_switches(pid: u32) -> io::Result<u32> {
  let result = fs::read_to_string(format!("/proc/{}/status", pid)).await?;
  let caps = if let Some(caps) = NV_CTXT_PAT.captures(&result) {
    caps
  } else {
    return io::Result::Err(std::io::ErrorKind::NotFound.into());
  };
  let nonvoluntary_ctxt_switches = if let Some(nonvoluntary_ctxt_switches) = caps.get(1) {
    nonvoluntary_ctxt_switches.as_str()
  } else {
    return io::Result::Err(std::io::ErrorKind::NotFound.into());
  };
  if let Ok(nonvoluntary_ctxt_switches) = nonvoluntary_ctxt_switches.parse::<u32>() {
    io::Result::Ok(nonvoluntary_ctxt_switches)
  } else {
    return io::Result::Err(std::io::ErrorKind::NotFound.into())
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn obserbe() {
    let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
    rt.block_on(async move {
      let result = super::observe_nonvoluntary_ctxt_switches(1).await;
      assert!(result.is_ok());
    });
  }
  #[test]
  fn collect() {
    let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
    rt.block_on(async move {
      let result = super::collect_processes(&["cargo"]).await;
      assert!(result.is_ok());
      let result = result.unwrap();
      assert!(result.len() >= 1);
    });
  }
}
