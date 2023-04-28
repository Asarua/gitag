extern crate chrono;

use chrono::{DateTime, Local};
use std::time::SystemTime;
use std::process::{Command};

fn main() {
  let time = DateTime::<Local>::from(SystemTime::now()).format("%Y-%m-%d_%H:%M:%S");
  let tag_name = format!("t_{}", time.to_string());
  let tag_cmd = Command::new("git").args(["tag", &tag_name]).output();

  match tag_cmd {
    Ok(new_tag_status) => {
      if new_tag_status.status.success() {
        let push_cmd = Command::new("git").args(["push", "origin", &tag_name]).output();
        
        match push_cmd {
          Ok(push_status) => {
            if push_status.status.success() {
              println!("git tag {} push success!", tag_name);
            } else {
              println!("error: {}", String::from_utf8_lossy(&push_status.stdout))
            }
          },
          Err(e) => panic!("{e}")
        }
      } else {
        println!("error: {}", String::from_utf8_lossy(&new_tag_status.stdout))
      }
    },
    Err(e) => panic!("{e}")
  }
}
