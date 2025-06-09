use std::collections::HashMap;

#[derive(Debug,Copy,Clone)]
pub struct Trigger {
  pub nn:u8,
  pub vel:u8
}

#[derive(Debug,Copy,Clone)]
pub struct SeqNote {
  pub nn:u8,
  pub vel:u8,
  pub duration:usize,
  pub legato:bool
}

#[derive(Debug,Default)]
pub struct LineBuilder {
  count:usize,
  max:usize,
  evs:HashMap<usize,Vec<SeqNote>>
}

impl LineBuilder {
  pub fn cr(&mut self) {
    self.max = self.count.max(self.max);
    self.count = 0;
  }

  pub fn done(self) -> (usize,HashMap<usize,Vec<SeqNote>>) {
    let max = self.count.max(self.max);
    (max,self.evs)
  }

  pub fn trig(&mut self, nn:u8,vel:u8,dur:usize,legato:bool) {
    let note = SeqNote{nn,vel,duration:dur + 1,legato};
    self.evs.entry(self.count).and_modify(|v|v.push(note)).or_insert(vec![note]);
    self.count += dur + 1
  }

  pub fn rests(&mut self,len:usize) {
    self.count += len
  }

  pub fn merge(&mut self,other_len:usize,other_notes:&HashMap<usize,Vec<SeqNote>>) {
    for i in 0..other_len {

      match other_notes.get(&i) {
        None => (),
        Some(v) => {
          self.evs.entry(self.count)
          .and_modify(|vs|vs.extend_from_slice(&v[..]))
          .or_insert(v.clone());
        }
      };

      self.count += 1;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_line_builder() {
    let mut bld = LineBuilder::default();
    bld.trig(10,10,4,true);
    bld.rests(5);
    bld.trig(11,11,4,false);
    bld.cr();
    bld.trig(12,12,4,false);
    bld.rests(15);
    let (len,map) = bld.done();
    assert_eq!(len,20);
    let trigs1 = map.get(&0).expect("should have a vec here");
    assert_eq!(trigs1.len(),2);
    assert!(matches!(trigs1[0],SeqNote{nn:10,vel:10,duration:5,legato:true}));
    assert!(matches!(trigs1[1],SeqNote{nn:12,vel:12,duration:5,legato:false}));

    let trigs2 = map.get(&10).expect("should have a vec here");
    assert!(matches!(trigs2[0],SeqNote{nn:11,vel:11,duration:5,legato:false}));
  }
}

#[derive(Debug,Clone)]
pub enum Val {
  Bar(usize,HashMap<usize,Vec<SeqNote>>),
  Trigger(Trigger),
}

impl std::fmt::Display for Val {
  fn fmt(&self, f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    match self {
      Val::Trigger(t) => write!(f,"({} {})",t.nn,t.vel),
      Val::Bar(len,evs) => {
        let mut lines = 1;
        let mut ln = 0;
        let mut ties = 0;
        let mut leg = false;
       
        while ln < lines {
          write!(f,"[")?;

          for idx in 0..*len {

            match evs.get(&idx) {
              Some(ts) => {
                lines = lines.max(ts.len());
                if ln < ts.len() {
                  let n = ts[ln];
                  write!(f,"({} {}) ",n.nn,n.vel)?;
                  ties = n.duration - 1;
                  leg = n.legato;
                }
                else {
                  write!(f,"- ")?
                }
              },
              None => {
                match ties {
                  0 => write!(f,"- ")?,
                  1 if leg => {
                    write!(f,"=>")?;
                    ties -= 1;
                  }
                  _ => {
                    write!(f,"= ")?;
                    ties -= 1;
                  }
                }
              }
            }
          };

          writeln!(f,"]")?;
          ln += 1;
        }
        Ok(())
      }
    }
  }
}
