use std::sync::mpsc::Sender;
use colored::Colorize;
use crate::{
  player::SeqCmd,
  lang::Action
};

pub struct Env {
  pub seq_tx:Sender<SeqCmd>
}

pub fn run_cmd(env:&mut Env,act:Action) -> Result<(),()> {
  match act {
    Action::Quit => {
      Err(())
    },
    Action::Seq(cmd) => {
      let _ = env.seq_tx.send(cmd).inspect_err(|e|{
        println!("{}: {}","seq_send failed".red(),e);
      });
      Ok(())
    }
  }
}
