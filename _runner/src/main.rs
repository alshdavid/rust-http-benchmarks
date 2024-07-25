use std::process::Command;
use std::process::Stdio;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
  let Ok(target) = std::env::var("PACKAGE") else {
    eprintln!("no PACKAGE supplied");
    std::process::exit(1);
  };

  let target_dir = std::env::current_dir()
    .unwrap()
    .parent()
    .unwrap()
    .join(&target);

  let (tx_done, rx_done) = channel::<bool>();
  let (tx_started, rx_started) = channel::<bool>();

  let h1 = thread::spawn(move || {
    let mut cmd = Command::new("bash");
    cmd
      .current_dir(target_dir)
      .arg("start.bash")
      .stdin(Stdio::null())
      .stdout(Stdio::piped())
      .stderr(Stdio::piped());

    let Ok(mut child) = cmd.spawn() else {
      tx_started.send(false).ok();
      return;
    };

    tx_started.send(true).ok();
    rx_done.recv().ok();
    child.kill().ok();
    child.wait().unwrap();
  });

  if !rx_started.recv().unwrap() {
    eprintln!("Failed to start process");
    std::process::exit(1);
  };

  let mut cmd = Command::new("just");
  cmd
    .arg("vegeta")
    .arg(&target)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());

  let Ok(mut child) = cmd.spawn() else {
    tx_done.send(true).ok();
    return;
  };

  child.wait().ok();
  tx_done.send(true).ok();
  h1.join().ok();
}
