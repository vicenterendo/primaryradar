use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
  static ref STDIN_TX: Mutex<Option<Sender<String>>> = Mutex::new(None);
  static ref STDOUT_RX: Mutex<Option<Receiver<String>>> = Mutex::new(None);
}

#[no_mangle]
pub unsafe extern "C" fn los(point_a: &[f64; 3], point_b: &[f64; 3]) -> bool {
  let tx = STDIN_TX.lock().unwrap();
  if let Some(ref tx) = *tx {
    let r = tx.send(format!("({}, {}, {}), ({}, {}, {})", point_a[0], point_a[1], point_a[2], point_b[0], point_b[1], point_b[2]));
    if !r.is_ok() {return false}
  } else {
      eprintln!("stdin_tx is not initialized");
  }
  let rx = STDOUT_RX.lock().unwrap();
  if let Some(ref rx) = *rx {
      let rr = rx.recv();
      if !rr.is_ok() {return false}
      else {
        let r = rr.unwrap();
        if r == "true" {return true}
        else {return false}
      }
  } else {
      eprintln!("stdout_rx is not initialized");
  }
  return false
}

#[no_mangle]
pub unsafe extern "C" fn load() {
  let (stdin_tx, stdin_rx) = mpsc::channel::<String>();
  let (stdout_tx, stdout_rx) = mpsc::channel::<String>();
  {
    let mut tx = STDIN_TX.lock().unwrap();
    *tx = Some(stdin_tx);
  }

  {
      let mut rx = STDOUT_RX.lock().unwrap();
      *rx = Some(stdout_rx);
  }
  let _ = thread::spawn(move || {
      let mut child = Command::new("primaryradarhelper.exe") // Replace "sh" with your interactive command
          .stdin(Stdio::piped())
          .stdout(Stdio::piped())
          .spawn()
          .expect("Failed to execute command");
      let mut child_stdin = child.stdin.take().expect("Failed to open stdin");
      let child_stdout = child.stdout.take().expect("Failed to open stdout");
      let stdout_tx_clone = stdout_tx.clone();
      thread::spawn(move || {
          let stdout_reader = io::BufReader::new(child_stdout);
          for line in stdout_reader.lines() {
              let line = line.expect("Failed to read line from stdout");
              stdout_tx_clone.send(line).expect("Failed to send line over channel");
          }
      });
      for line in stdin_rx {
          writeln!(child_stdin, "{}", line).expect("Failed to write to stdin");
      }
      child.wait().expect("Child process wasn't running");
  });
//    let a = mtest();
//    println!("{}", a);

}