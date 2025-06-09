use super::{
  sqt::Sequence,
  voicer::Voicer
};

use lang::SeqNote;

#[derive(Debug,Default,Copy,Clone,PartialEq,Eq)]
pub enum Transport {
  #[default]
  Stop,
  StopSync,
  Play,
  PlaySync
}

pub struct Slot {
  pub out:usize,
  current:Option<Box::<dyn Sequence>>,
  tpt:Transport,
  tick:usize,
  step:usize,
  tps:usize,
  sync_pt:usize
}

fn play_step(step:usize, tps:usize, seq:&mut dyn Sequence,v:&mut Voicer) -> Result<(),TickError> {
  if let Some(events) = seq.events(step) {
    for sqn in events.iter() {
      v.play_note(sqn.nn,sqn.vel,sqn.vel,sqn.duration * tps ,sqn.legato)
      .map_err(|_|TickError::Bad)?;
    }
  }

  Ok(())
}

impl Slot {
  pub fn tick(&mut self,v:Option<&mut Voicer>) -> Result<(),TickError> {
    if let (Some(seq),Transport::Play) = (&mut self.current,self.tpt) {
      let err = if let (0,Some(voice)) = (self.tick,v) {
        play_step(self.step,self.tps,seq.as_mut(),voice)
      }
      else {
        Ok(())
      };
  
      self.tick += 1;

      if self.tick % self.tps == 0 {
        self.tick = 0;
        self.step = (self.step + 1) % seq.len()
      }

      err
    }
    else {
      Ok(())
    }
  }

  pub fn sync(&mut self,sync_ticks:usize) {
    if self.sync_pt == 0 {
      return
    }

    if sync_ticks % self.sync_pt == 0 {
      match self.tpt {
        Transport::StopSync => self.play(),
        Transport::PlaySync => self.stop(),
        _ => ()
      }
    }
  }

  pub fn assign(&mut self, seq:Box::<dyn Sequence>) {
    self.current = Some(seq);
    self.tick=0;
    self.step=0;
  }

  pub fn play(&mut self) {
    self.tpt = Transport::Play
  }

  pub fn stop(&mut self) {
    self.tpt = Transport::Stop;
    self.tick=0;
    self.step=0;
  }
  
  pub fn toggle(&mut self) {
    match (self.tpt,self.sync_pt) {
      (Transport::Stop,0) => self.play(),
      (Transport::Stop,_) => self.tpt = Transport::PlaySync,
      (Transport::StopSync,_) => self.tpt = Transport::Play,
      (Transport::Play,0) => self.stop(),
      (Transport::Play,_) => self.tpt = Transport::PlaySync,
      (Transport::PlaySync,_) => self.tpt = Transport::Play
    }
  }

  pub fn state(&self) -> SlotState {
    if self.current.is_none() {
      return SlotState::Empty
    };

    match self.tpt {
      Transport::Stop => SlotState::Stopped,
      Transport::Play => SlotState::Playing,
      Transport::StopSync => SlotState::StopSync,
      Transport::PlaySync => SlotState::PlaySync
    }
  }

  pub fn set_div(&mut self,new_div:usize) {
    self.tps = new_div;
  }
  
  pub fn set_sync(&mut self,new_sync:usize) {
    self.sync_pt = new_sync;
    if self.sync_pt == 0 {
      match self.tpt {
        Transport::StopSync => self.play(),
        Transport::PlaySync => self.stop(),
        _ => ()
      }
    }
  }
}

impl Default for Slot {
  fn default() -> Self {
    Self {
      out:0,
      current:None,
      tpt:Transport::Stop,
      tick:0,
      step:0,
      tps:6,
      sync_pt:0
    }
  }
}

#[derive(Debug)]
pub enum TickError {
  Bad
}

pub enum SlotState {
  Empty,
  Stopped,
  StopSync,
  Playing,
  PlaySync
}
