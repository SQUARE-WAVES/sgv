use rustyline::error::ReadlineError;
use crate::player::Rt;

mod helper;
pub mod editor;

pub fn start_threaded(rt:Rt) -> Result<std::thread::JoinHandle<()>,Err> {
  let mut terp = lang::Env::new(rt);
  let mut ed = editor::new().map_err(Err::ReadLine)?;

  let handle = std::thread::spawn(move||{
    loop {
      if let Err(e) = editor::lp(&mut ed,&mut terp) {
        println!("quitting repl: {}",e);
        break;
      }
    }
  });

  Ok(handle)
}

pub fn start_main(rt:Rt) -> Result<(),Box<dyn std::error::Error>> {
  let mut terp = lang::Env::new(rt);
  let mut ed = editor::new().map_err(Err::ReadLine)?;

  loop {
    if let Err(e) = editor::lp(&mut ed,&mut terp) {
      println!("quitting repl: {}",e);
      break;
    }
  }

  Ok(())
}


#[derive(Debug)]
pub enum Err {
  Quit,
  ReadLine(ReadlineError)
}

impl std::fmt::Display for Err {
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    match self {
      Self::Quit => write!(f,"Quittin time"),
      Self::ReadLine(e) => e.fmt(f)
    }
  }
}

impl std::error::Error for Err {}
