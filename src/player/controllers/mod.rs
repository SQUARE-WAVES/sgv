use std::sync::mpsc::Sender;

use super::SeqCmd;

//consider caching grid state
//to prevent sending off messages
//all the dang time
pub struct Lpadout {
  port: midir::MidiOutputConnection
}

impl Lpadout {
  pub fn new(p:midir::MidiOutputConnection) -> Self {
    Self {
      port:p
    }
  }

  pub fn set_grid<T:Into<u8>>(&mut self,row:usize,col:usize,color:T) -> Result<(),()> {
    let row = row.min(7) as u8;
    let col = col.min(7) as u8;
    let nn = (row << 4) | col;
    self.port.send(&[0x90,nn,color.into()]).map_err(|_|())?;
    Ok(())
  }

  /*
  pub fn close(self) {
    self.port.close();
  }

  pub fn set_side<T:Into<u8>>(&mut self,row:usize,color:T) -> Result<(),()> {
    let row = row.min(7) as u8;
    let nn = (row << 4) | 8;
    self.port.send(&[0x90,nn,color.into()]).map_err(|_|())?;
    Ok(())
  }

  pub fn set_top<T:Into<u8>>(&mut self,col:usize,color:T) -> Result<(),()> {
    let col = col.min(7) as u8;
    let nn = 104+col;
    self.port.send(&[0xB0,nn,color.into()]).map_err(|_|())?;
    Ok(())
  }*/
}

pub type Lpadin = midir::MidiInputConnection<Sender<SeqCmd>>;

fn lpad_cb(_time:u64,msg:&[u8],port:&mut std::sync::mpsc::Sender<SeqCmd>) {
  let [status,nn,vel] = msg else { todo!("weird midi message from launchpad") };

  match status {
    /*0xB0 => {
      let col = nn-104;
      let hit = *vel != 0;
    },*/

    0x90 => {
      let row = (nn & 0xF0)>>4;
      let col = nn & 0x0F;
      let hit = *vel !=0;

      if col == 8 {
        //println!("right side hit, row:{} on_off:{}",row,hit);
      }
      else {
        let slot = (row*8) + col;
        if hit {
          let _ = port.send(SeqCmd::ToggleSlot(slot.into()))
          .inspect_err(|_|println!("lp port broke"));
        }
      }
    },

    _ => {
      todo!("weird status byte from launchpad");
    }
  };
}

fn lpad_in(tx:Sender<SeqCmd>) -> Result<Lpadin,LpadError> {
  let midi_in = midir::MidiInput::new("Lpad in").map_err(|_|LpadError::InPort)?;
  let ports = midi_in.ports();
  let in_port = ports.iter().find(|p|{
    midi_in.port_name(p).is_ok_and(|nm| matches!(nm.as_ref(),"Launchpad Mini"))
  })
  .ok_or(LpadError::InFind)?;

  let conn = midi_in.connect(in_port,"lpad in connection",lpad_cb,tx)
  .map_err(|_|LpadError::InConnect)?;

  Ok(conn)
}

fn lpad_out() -> Result<Lpadout,LpadError> {
  let midi_out = midir::MidiOutput::new("Lpad out").map_err(|_|LpadError::OutPort)?;
  let ports = midi_out.ports();
  let out_port = ports.iter().find(|p|{
    midi_out.port_name(p).is_ok_and(|nm| matches!(nm.as_ref(),"Launchpad Mini"))
  })
  .ok_or(LpadError::OutFind)?;

  let conn = midi_out.connect(out_port,"lpad out connection")
  .map_err(|_|LpadError::OutConnect)?;

  Ok(Lpadout::new(conn))
}

pub fn find_lpad(tx:Sender<SeqCmd>) -> Result<(Lpadout,Lpadin),LpadError> {
  let out = lpad_out()?;
  let inp = lpad_in(tx)?;
  Ok((out,inp))
}

pub enum Color {
  Off,
  Red1,
  Red2,
  Red3,
  Green1,
  Green2,
  Green3,
  Orange1,
  Orange2,
  Orange3
}

impl From<Color> for u8 {
  fn from(x:Color) -> u8 {
    match x {
      Color::Off => 0x00,
      Color::Red1 => 0x01,
      Color::Red2 => 0x02,
      Color::Red3 => 0x03,
      Color::Green1 => 0x10,
      Color::Green2 => 0x20,
      Color::Green3 => 0x30,
      Color::Orange1 => 0x11,
      Color::Orange2 => 0x22,
      Color::Orange3 => 0x33
    }
  }
}

#[derive(Debug)]
pub enum LpadError {
  InPort,
  InFind,
  InConnect,
  OutPort,
  OutFind,
  OutConnect
}

impl std::fmt::Display for LpadError {
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    match self {
      Self::InPort => write!(f,"couldn't make an input"),
      Self::InFind => write!(f,"couldn't find a launchpad"),
      Self::InConnect => write!(f,"coudln't connect to the input"),
      Self::OutPort => write!(f,"couldn't make an output"),
      Self::OutFind => write!(f,"couldn't find a launchpad"),
      Self::OutConnect => write!(f,"couldn't connect to output")
    }
  }
}

impl std::error::Error for LpadError {}

