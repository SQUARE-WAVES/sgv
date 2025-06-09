use std::error::Error;
mod sequence_types;
mod repl;
mod player;

fn main() -> Result<(),Box<dyn Error>> {
  let (tx,_rx,handle) = player::start();

  repl::start_main(player::Rt::new(tx.clone()))?;
 
  let _ = tx.send(player::SeqCmd::Quit);
  handle.join().expect("join broke");
  Ok(())
}
