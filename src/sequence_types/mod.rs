use std::collections::HashMap;
use lang::SeqNote;

pub trait Sequence:Send {
  fn events(&mut self,step:usize) -> Option<&[SeqNote]>;
  fn len(&self) -> usize;
}

#[derive(Debug,Default,Clone)]
pub struct MapSeq {
  notes:HashMap<usize,Vec<SeqNote>>,
  len:usize
}

impl MapSeq {
  pub fn new(notes:HashMap<usize,Vec<SeqNote>>,len:usize) -> Self {
    Self {
      notes,
      len
    }
  }
}

impl Sequence for MapSeq {
  fn len(&self) -> usize {
    self.len
  }

  fn events(&mut self,step:usize) -> Option<&[SeqNote]> {
    self.notes.get(&step).map(|v|&v[..])
  }
}

impl From<(usize,HashMap<usize,Vec<SeqNote>>)> for MapSeq {
  fn from((len,notes):(usize,HashMap<usize,Vec<SeqNote>>)) -> Self {
    Self::new(notes,len)
  }
}
