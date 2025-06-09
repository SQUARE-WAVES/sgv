use std::{
  collections::HashMap,
  sync::mpsc::{
    channel,
    Sender,
    Receiver
  }
};

use crate::sequence_types as sqt;

mod time;
mod voicer;
mod slot;
mod sequencer;
mod controllers;

use {
  sequencer::Sequencer,
  sqt::Sequence
};

pub use voicer::Voicer;

pub enum SeqCmd {
  PushOutput(Voicer),
  SetBpm(usize),
  AssignSlot(usize,Box<dyn Sequence>),
  AssignOutput(usize,usize),
  PlaySlot(usize),
  StopSlot(usize),
  SetSync(usize,usize),
  SetDiv(usize,usize),
  ToggleSlot(usize),
  Launchpad(controllers::Lpadout),
  
  Quit
}

pub enum SeqRsp {
  Done,
  Failed(usize),
}

pub struct Rt {
  cmd_tx:Sender<SeqCmd>,
  lp_ins:Vec<controllers::Lpadin>
}

impl Rt {
  pub fn new(cmd_tx:Sender<SeqCmd>) -> Self {
    Self{
      cmd_tx,
      lp_ins:vec![]
    }
  }
}

impl lang::Runtime for Rt {
  fn assign_slot(&mut self,snum:usize,len:usize,evs:HashMap<usize,Vec<lang::SeqNote>>){
    let seq : Box<sqt::MapSeq> = Box::new((len,evs).into());
    self.cmd_tx.send(SeqCmd::AssignSlot(snum,seq)).expect("send failed");
  }

  fn set_bpm(&mut self,bpm_num:usize) {
    self.cmd_tx.send(SeqCmd::SetBpm(bpm_num)).expect("send failed");
  }

  fn set_div(&mut self,slot:usize,div_num:usize) {
    self.cmd_tx.send(SeqCmd::SetDiv(slot,div_num)).expect("send failed");
  }

  fn set_sync(&mut self,slot:usize,sync_num:usize) {
    self.cmd_tx.send(SeqCmd::SetSync(slot,sync_num)).expect("send failed");
  }

  fn play_slot(&mut self,slot:usize) {
    self.cmd_tx.send(SeqCmd::PlaySlot(slot)).expect("send failed");
  }

  fn stop_slot(&mut self,slot:usize){ 
    self.cmd_tx.send(SeqCmd::StopSlot(slot)).expect("send failed");
  }
  
  fn list_outs(&mut self) {
    let midi_out = midir::MidiOutput::new("lister").expect("failed to create midi output");
    let ports = midi_out.ports();
    for (i,p) in ports.iter().enumerate() {
      match midi_out.port_name(p) {
        Ok(n) => println!("{}: {}",i,n),
        Err(e) => println!("{}: Couldn't get name:{}",i,e)
      }
    }
  }

  fn list_lps(&mut self) {
    let midi_in = midir::MidiInput::new("lister").expect("failed to create midi input");
    let ports = midi_in.ports();
    for (i,p) in ports.iter().enumerate() {
      match midi_in.port_name(p) {
        Ok(s) if s == "Launchpad Mini" => println!("{}: {}",i,s),
        Ok(_) => (),
        Err(_) => ()
      }
    }
  }

  fn open_out(&mut self,out_num:usize,channel:u8) {
    let midi_out = midir::MidiOutput::new("new conn").expect("failed to create midi output");
    let ports = midi_out.ports();
    let p = midi_out.connect(&ports[out_num],"why?").expect("couldn't connect to requested port");
    let v = crate::player::Voicer::new(p,channel).expect("failed to create voicer");

    self.cmd_tx.send(SeqCmd::PushOutput(v)).expect("send failed");
  }

  fn set_output(&mut self,lot:usize,out:usize) {
    self.cmd_tx.send(SeqCmd::AssignOutput(lot,out)).expect("send failed");
  }

  fn open_lp(&mut self,_lp_num:usize) {
    match controllers::find_lpad(self.cmd_tx.clone()) {
      Ok((lpout,lpin)) => {
        self.lp_ins.push(lpin);
        self.cmd_tx.send(SeqCmd::Launchpad(lpout)).expect("send failed");
      },
      Err(e) => println!("couldn't open lp {:?}",e)
    }
  }
}

fn seq_thread(rx:Receiver<SeqCmd>,tx:Sender<SeqRsp>,mut seq:Sequencer) {
  let _ = tx.send(SeqRsp::Done);

  let loop_res : Result<(),usize> = 'main: loop {
    match seq.tick() {
      Ok(()) => (),
      Err(n) => break Err(n)
    };

    for msg in rx.try_iter() {
      match msg {
        SeqCmd::SetBpm(bpm) => { seq.set_bpm(bpm); },
        SeqCmd::PushOutput(v) => { seq.push_output(v); },
        SeqCmd::AssignSlot(n,bds) => { seq.assign_slot(n,bds); },
        SeqCmd::AssignOutput(n,u) => { seq.assign_slot_output(n,u); },
        SeqCmd::PlaySlot(n) => { seq.play_slot(n); },
        SeqCmd::StopSlot(n) => { seq.stop_slot(n); }
        SeqCmd::SetSync(n,sync) => {seq.set_sync(n,sync);},
        SeqCmd::SetDiv(n,div) => {seq.set_div(n,div);},
        SeqCmd::ToggleSlot(n) => { seq.toggle_slot(n); }
        SeqCmd::Launchpad(lp) => { seq.add_lp(lp); }
        SeqCmd::Quit => { 
          seq.kill_all();
          break 'main Ok(())
        }
      }
    }

    seq.wait();
  };

  let _ = match loop_res {
    Ok(()) => tx.send(SeqRsp::Done),
    Err(e) => tx.send(SeqRsp::Failed(e))
  };
}

pub fn start() -> (Sender<SeqCmd>,Receiver<SeqRsp>,std::thread::JoinHandle<()>) {
  let (cmd_tx,cmd_rx) = channel::<SeqCmd>();
  let (rsp_tx,rsp_rx) = channel::<SeqRsp>();

  let handle = std::thread::spawn(move ||{
    seq_thread(cmd_rx,rsp_tx,Sequencer::init(120,64,None));
  });

  (cmd_tx,rsp_rx,handle)
}
