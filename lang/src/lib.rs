use std::collections::HashMap;

mod err;
mod env;
mod seq_types;
mod lex;
mod bar;
mod keywords;
mod parse_fsm;

pub use {
  env::Env,
  err::Error,
  seq_types::SeqNote,
  lex::{Token,Lexer,ExpectErr},
};

pub trait Runtime {
  fn assign_slot(&mut self,snum:usize,len:usize,evs:HashMap<usize,Vec<SeqNote>>);
  fn set_bpm(&mut self,bpm_num:usize);
  fn set_div(&mut self,slot:usize,div_num:usize);
  fn set_sync(&mut self,slot:usize,sync_num:usize);
  fn set_output(&mut self,slot:usize,out_num:usize);
  fn play_slot(&mut self,slot:usize);
  fn stop_slot(&mut self,slot:usize);
  fn list_outs(&mut self);
  fn list_lps(&mut self);
  fn open_out(&mut self,out_num:usize,channel:u8);
  fn open_lp(&mut self,lp_num:usize);
}

pub fn parse<'a,T:Runtime>(src:&'a str, env:&mut Env<T>) -> Result<(),err::Error<'a>> {
  let mut lx = Lexer::new(src);
  match parse_fsm::root(&mut lx,env) {
    Ok(()) => Ok(()),
    Err(err::ParseError::Msg(msg)) => {
      let (src,s,e) = lx.done();
      Err(Error::Root(src,s,e,msg))
    },
    Err(err::ParseError::SubFile(msg,subtxt,s,e)) => { 
      let (src,ns,ne) = lx.done();
      Err(Error::Sub(&src[ns..ne],subtxt,s,e,msg))
    }
  }
}

mod test_utils {
  use super::*;

  pub struct NullRt{}

  impl Runtime for NullRt {
    fn assign_slot(&mut self,_:usize,_:usize,_:HashMap<usize,Vec<SeqNote>>) {}
    fn set_bpm(&mut self,_:usize) {}
    fn set_div(&mut self,_:usize,_:usize) {}
    fn set_sync(&mut self,_:usize,_:usize) {}
    fn set_output(&mut self,_:usize,_:usize) {}
    fn play_slot(&mut self,_:usize) {}
    fn stop_slot(&mut self,_:usize) {}
    fn list_outs(&mut self) {}
    fn list_lps(&mut self) {}
    fn open_out(&mut self,_:usize,_:u8) {}
    fn open_lp(&mut self,_:usize) {}
  }
}
