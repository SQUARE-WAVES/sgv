use super::{
  voicer::Voicer,
  time::TimeBuddy,
  slot::{
    Slot,
    SlotState
  },
  sqt::Sequence,
  controllers::{
    Lpadout,
    Color
  }
};

pub struct Sequencer {
  outs:Vec<Voicer>,
  slots:Vec<Slot>,
  timer:TimeBuddy,
  lp:Option<Lpadout>,
  sync_ticks:usize
}

impl Sequencer {
  pub fn init(bpm:usize,slot_count:usize,lp:Option<Lpadout>) -> Self {
    let mut slots = vec![];
    for _ in 0..slot_count {
      slots.push(Slot::default());
    };

    Sequencer {
      outs:vec![],
      slots,
      timer: TimeBuddy::new(bpm),
      lp,
      sync_ticks:0
    }
  }

  pub fn set_bpm(&mut self,bpm:usize) {
    self.timer = TimeBuddy::new(bpm)
  }

  pub fn tick(&mut self) -> Result<(),usize> {
    //kill all the non-legato notes
    for voicer in self.outs.iter_mut() {
      voicer.pre_tick();
    }

    for slot in self.slots.iter_mut() {
      slot.sync(self.sync_ticks);
      let out = self.outs.get_mut(slot.out);
      let _ = slot.tick(out).inspect_err(|e|println!("slot output error: {:?}",e));
    }

    for voicer in self.outs.iter_mut() {
      voicer.tick();
    }

    self.sync_ticks += 1;
    if self.sync_ticks == 96*4 {
      self.sync_ticks = 0;
    }

    Ok(())
  }

  pub fn kill_all(&mut self) {
    self.slots.iter_mut().for_each(|s|s.stop());
    self.outs.iter_mut().for_each(|v|v.kill_all());
  }

  pub fn wait(&mut self) {
    self.timer.wait()
  }

  pub fn assign_slot(&mut self, slot:usize,seq:Box<dyn Sequence>) {
    if let Some(s) = self.slots.get_mut(slot) {
      s.assign(seq)
    }
  }

  pub fn assign_slot_output(&mut self, slot:usize,out:usize) {
    if let Some(s) = self.slots.get_mut(slot) {
      s.out = out;
    }
  }

  pub fn push_output(&mut self,v:Voicer) -> usize {
    self.outs.push(v);
    self.outs.len()
  }

  pub fn set_sync(&mut self,slot:usize,sync_num:usize) {
    if let Some(s) = self.slots.get_mut(slot) {
      s.set_sync(sync_num);
      self.sync_lp();
    }
  }
  
  pub fn set_div(&mut self,slot:usize,sync_num:usize) {
    if let Some(s) = self.slots.get_mut(slot) {
      s.set_div(sync_num);
    }
  }

  pub fn play_slot(&mut self,slot:usize) {
    if let Some(s) = self.slots.get_mut(slot) {
      s.play();
      self.sync_lp();
    }
  }

  pub fn stop_slot(&mut self,slot:usize) {
    if let Some(s) = self.slots.get_mut(slot) {
      s.stop();
      self.sync_lp();
    }
  }

  pub fn toggle_slot(&mut self,slot:usize) {
    if let Some(s) = self.slots.get_mut(slot) {
      s.toggle();
      self.sync_lp();
    }
  }

  pub fn add_lp(&mut self,lp:Lpadout) {
    self.lp = Some(lp);
    self.sync_lp();
  }

  pub fn drop_lp(&mut self) {
    self.lp = None
  }

  pub fn sync_lp(&mut self) {
    let Some(lp) = self.lp.as_mut() else {return};

    for (i,s) in self.slots.iter().enumerate() {
      let row = i/8;
      let col = i%8;

      let _ = match s.state() {
        SlotState::Empty => lp.set_grid(row,col,Color::Off),
        SlotState::Stopped => lp.set_grid(row,col,Color::Red3),
        SlotState::StopSync => lp.set_grid(row,col,Color::Red1),
        SlotState::Playing => lp.set_grid(row,col,Color::Green3),
        SlotState::PlaySync=> lp.set_grid(row,col,Color::Green1),
      };
    }
  }
}
