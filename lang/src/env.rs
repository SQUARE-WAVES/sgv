use std::collections::HashMap;

use super::{
  Lexer,
  parse_fsm,
  Runtime,
  seq_types::{SeqNote,Val},
  err
};

#[derive(Debug)]
pub struct Env<T> {
  stack:Vec<HashMap<String,Val>>,
  rt:T
}

impl<T:Runtime> Env<T> {
  pub fn new(rt:T) -> Self {
    Self {
      stack:vec![HashMap::new()],
      rt
    }
  }

  pub fn lookup(&self,nm:&str) -> Option<&Val> {
    for scope in self.stack.iter().rev() {
      if let Some(val) = scope.get(nm) {
        return Some(val)
      }
    }
    None
  }

  pub fn step_in(&mut self) {
    self.stack.push(HashMap::new())
  }

  pub fn step_out(&mut self) {
    self.stack.pop();
  }

  pub fn set(&mut self,nm:&str,val:Val) {
    let stack_top = self.stack.iter_mut().next_back().expect("all scopes gone");
    stack_top.insert(String::from(nm),val);
  }

  pub fn do_file<P:AsRef<std::path::Path>>(&mut self,file:P) -> Result<(),err::ParseError> {
    let ftxt = std::fs::read_to_string(file).map_err(|_|{
      err::ParseError::Msg("couldn't open file")
    })?;

    let mut sub_lex = Lexer::new(&ftxt[..]);
    let parse_result = parse_fsm::root(&mut sub_lex,self);
    let (_,start,end) = sub_lex.done();

    match parse_result {
      Ok(_) => Ok(()),
      Err(err::ParseError::Msg(msg)) => Err(err::ParseError::SubFile(msg,ftxt,start,end)),
      Err(e@err::ParseError::SubFile(_,_,_,_)) => Err(e)
    }
  }

  pub fn assign_slot(&mut self,snum:usize,len:usize,evs:HashMap<usize,Vec<SeqNote>>) {
    self.rt.assign_slot(snum,len,evs);
  }

  pub fn set_bpm(&mut self,bpm_num:usize) {
    self.rt.set_bpm(bpm_num);
  }

  pub fn set_div(&mut self,slot:usize,div_num:usize) {
    self.rt.set_div(slot,div_num);
  }

  pub fn set_sync(&mut self,slot:usize,sync_num:usize) {
    self.rt.set_sync(slot,sync_num);
  }

  pub fn play_slot(&mut self,slot_num:usize) {
    self.rt.play_slot(slot_num);
  }

  pub fn stop_slot(&mut self,slot_num:usize) {
    self.rt.stop_slot(slot_num)
  }

  pub fn list_outs(&mut self) {
    self.rt.list_outs()
  }

  pub fn list_lps(&mut self) {
    self.rt.list_lps()
  }
  
  pub fn open_out(&mut self,out_num:usize,channel:u8) {
    self.rt.open_out(out_num,channel)
  }

  pub fn set_output(&mut self,slot:usize,output:usize) {
    self.rt.set_output(slot,output)
  }
  
  pub fn open_lp(&mut self,lp_num:usize) {
    self.rt.open_lp(lp_num)
  }
}
