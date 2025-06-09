use midir::MidiOutputConnection as Conn;

pub struct Voicer {
  conn:midir::MidiOutputConnection,
  channel:u8,
  buffer:Vec<(u8,u8,usize,bool)>
}

impl Voicer {
  pub fn new(conn:Conn,channel:u8) -> Result<Self,VoiceError> {
    Ok(Self{
      conn,
      channel,
      buffer:Vec::with_capacity(128)
    })
  }

  pub fn pre_tick(&mut self) {
    self.buffer.iter_mut().for_each(|msg| {
      let (nn,ov,dur,leg) = msg;

      if let (0,false) = (*dur,*leg) {
        self.conn.send(&[0x80|self.channel,*nn,*ov]).expect("note off failed");
      }
    });
  }

  pub fn tick(&mut self) {
    self.buffer.retain_mut(|msg| {
      let (nn,ov,dur,leg) = msg;

      match (*dur,*leg) {
        (0,true) => {
          self.conn.send(&[0x80|self.channel,*nn,*ov]).expect("note off failed");
          false
        },

        (0,false) => false,

        _ => {
          *msg = (*nn,*ov,*dur-1,*leg);
          true
        }
      }
    });
  }

  pub fn kill_all(&mut self) {
    self.buffer.retain_mut(|msg| {
      let (nn,ov,_,_) = msg;
      self.conn.send(&[0x80|self.channel,*nn,*ov]).expect("note off failed");
      false
    });
  }

  pub fn play_note(&mut self,nn:u8,v:u8,ov:u8,dur:usize,leg:bool) -> Result<(),VoiceError> {
    self.conn.send(&[0x90 | self.channel,nn,v]).map_err(|_|VoiceError::NoteOnFailed)?;
    self.buffer.push((nn,ov,dur,leg));
    Ok(())
  }
}

impl Drop for Voicer {
  fn drop(&mut self) {
    self.kill_all();
  }
}

#[derive(Debug)]
pub enum VoiceError {
  NoteOnFailed,
  NoteOffFailed,
  CouldntFindDevice,
  NoMidi,
  NoConnect,
  NoDevice
}

impl std::fmt::Display for VoiceError {
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    write!(f,"its a voiceerror")
  }
}

impl std::error::Error for VoiceError{}
