pub struct TimeBuddy {
  timer:adi_clock::Timer
}

//this goes from BPM to seconds per tick (1/24 of a beat)
const fn bpm_to_timing(bpm:usize) -> f32 {
  60.0/(24.0 * (bpm as f32))
}

impl TimeBuddy {
  pub fn new(bpm:usize) -> Self {
    let timer = adi_clock::Timer::new(bpm_to_timing(bpm));
    Self {
      timer
    }
  }

  pub fn wait(& mut self) {
    self.timer.wait();
  }
}
