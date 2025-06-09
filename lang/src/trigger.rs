use pekoms::Parser;

use super::{
  Token,
  Lexer,
  err
};

#[derive(Debug,Copy,Clone)]
pub struct Note {
  pub val:u8,
  pub span:(usize,usize)
}

#[derive(Debug,Copy,Clone)]
pub struct Vel {
  pub val:u8,
  pub span:(usize,usize)
}

#[derive(Debug,Clone)]
pub struct Trigger {
  pub note:Note,
  pub vel:Vel,
  pub span:(usize,usize)
}

pub fn trigger(lx:Lexer) -> Result<(Trigger,Lexer),err::Error> {
  let open = Token::OpenParen.map_err(|lx|err::trigger(lx,ErrCause::NoStart));
  let close = Token::CloseParen.map_err(|lx|err::trigger(lx,ErrCause::NoEnd));

  (open,note,vel,close).map(|((_,st_span),note,vel,(_,e_span))|{
    Trigger {
      note,
      vel,
      span:(st_span.start,e_span.end)
    }
  }).parse(lx)
}

fn note(mut lx:Lexer) -> Result<(Note,Lexer),err::Error> {
  match lx.next() {
    Some(Ok(Token::Note)) => {
      let txt = lx.slice();
      let span = (lx.span().start,lx.span().end);
      let val = note_to_n(txt);
      Ok( (Note{val,span},lx) )
    },

    Some(Ok(Token::Digits)) => {
      let txt = lx.slice();
      let span = (lx.span().start,lx.span().end);
      let val = match txt.parse::<u8>() {
        Ok(n) if (0..128).contains(&n) => n,
        Ok(_) => return Err(err::trigger(lx,ErrCause::NoteOutOfRange)),
        Err(_) =>return  Err(err::trigger(lx,ErrCause::Note)),
      };

      Ok( (Note{val,span},lx) )
    },

    Some(Ok(_)) => Err(err::trigger(lx,ErrCause::Note)),

    _ => Err(err::lex(lx))
  }
}

fn vel(mut lx:Lexer) -> Result<(Vel,Lexer),err::Error> {
  match lx.next() {
    Some(Ok(Token::Digits)) => {
      let txt = lx.slice();
      let span = (lx.span().start,lx.span().end);
      let val = match txt.parse::<u8>() {
        Ok(n) if (0..128).contains(&n) => n,
        Ok(_) => return Err(err::trigger(lx,ErrCause::VelOutOfRange)),
        Err(_) => return Err(err::trigger(lx,ErrCause::Vel))
      };

      Ok( (Vel{val,span},lx) )
    },
    Some(Ok(_)) => Err(err::trigger(lx,ErrCause::Vel)),
    _ => Err(err::lex(lx))
  }
}

fn note_to_n(txt:&str) -> u8 {
  let pc : isize = match &txt[0..1] {
    "c"|"C" => 0,
    "d"|"D" => 2,
    "e"|"E" => 4,
    "f"|"F" => 5,
    "g"|"G" => 7,
    "a"|"A" => 9,
    "b"|"B" => 11,
    _ => unreachable!("invalid pitch class in note")
  };

  let modifier : isize = match &txt[1..2] {
    "b" => -1,
    "_" => 0,
    "#" => 1,
    _ => unreachable!("shouldn't have invalid notes")
  };

  let octave = match &txt[2..3].parse::<isize>() {
    Ok(i) => *i,
    Err(_) => unreachable!("shouldn't have invalid notes here")
  };

  let val = 24 + pc + (12*octave) + modifier;
  val as u8
}

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub enum ErrCause{
  NoStart,
  Note,
  NoteOutOfRange,
  VelOutOfRange,
  Vel,
  NoEnd
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_trigger() {
    let input = "(C#2 127)";
    let lx = Lexer::new(input);
    let (v,mut r) = trigger(lx).expect("should come out right");
    assert!(r.next().is_none());
    assert_eq!(v.span,(0,input.len()));
    assert_eq!(v.note.val,49);
    assert_eq!(v.note.span,(1,4));
    assert_eq!(v.vel.val,127);
    assert_eq!(v.vel.span,(5,8));

    let input = "(    66 44     )";
    let lx = Lexer::new(input);
    let (v,mut r) = trigger(lx).expect("should come out right");
    assert!(r.next().is_none());
    assert_eq!(v.span,(0,input.len()));
    assert_eq!(v.note.val,66);
    assert_eq!(v.note.span,(5,7));
    assert_eq!(v.vel.val,44);
    assert_eq!(v.vel.span,(8,10));

    let input = "44 127)";
    let lx = Lexer::new(input);
    let e = trigger(lx).expect_err("should fail");
    assert_eq!(e.cause,err::Cause::Trigger(ErrCause::NoStart));

    let input = "(44 127";
    let lx = Lexer::new(input);
    let e = trigger(lx).expect_err("should fail");
    assert_eq!(e.cause,err::Cause::Trigger(ErrCause::NoEnd));

    let input = "(#? 127)";
    let lx = Lexer::new(input);
    let e = trigger(lx).expect_err("should fail");
    assert_eq!(e.cause,err::Cause::Lex);

    let input = "(breep 127)";
    let lx = Lexer::new(input);
    let e = trigger(lx).expect_err("should fail");
    assert_eq!(e.cause,err::Cause::Trigger(ErrCause::Note));

    let input = "(200 127)";
    let lx = Lexer::new(input);
    let e = trigger(lx).expect_err("should fail");
    assert_eq!(e.cause,err::Cause::Trigger(ErrCause::NoteOutOfRange));

    let input = "(66 corn)";
    let lx = Lexer::new(input);
    let e = trigger(lx).expect_err("should fail");
    assert_eq!(e.cause,err::Cause::Trigger(ErrCause::Vel));

    let input = "(66 129)";
    let lx = Lexer::new(input);
    let e = trigger(lx).expect_err("should fail");
    assert_eq!(e.cause,err::Cause::Trigger(ErrCause::VelOutOfRange));
  }
}
