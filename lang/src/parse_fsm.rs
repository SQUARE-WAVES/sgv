use super::{Lexer,Token,Runtime,Env,bar,keywords,err};
use crate::seq_types::{Val,Trigger,LineBuilder};

pub fn root<RT:Runtime>(lx:&mut Lexer,env:&mut Env<RT>) -> Result<(),err::ParseError> {
  loop {
    match lx.lookahead() {
      Token::Eol => { 
        lx.advance();
      }
      Token::Sym | Token::Digits => {
        lx.advance();
        assignment(lx,env)?
      },
      Token::OpenAng => slot_assignment(lx,env)?,
      Token::Bpm => keywords::bpm(lx,env)?,
      Token::Div => keywords::div(lx,env)?,
      Token::Sync => keywords::sync(lx,env)?,
      Token::Play => keywords::play(lx,env)?,
      Token::Stop => keywords::stop(lx,env)?,
      Token::ListOuts => keywords::list_outs(lx,env)?,
      Token::OpenOut => keywords::open_out(lx,env)?,
      Token::SetOut => keywords::set_out(lx,env)?,
      Token::FindLps => keywords::list_lps(lx,env)?,
      Token::OpenLp => keywords::open_lp(lx,env)?,
      Token::DoFile => keywords::do_file(lx,env)?,

      Token::Eof => return Ok(()),
      _ => return Err("unknown symbol".into())
    }
  }
}

fn assignment<RT:Runtime>(lx:&mut Lexer,env:&mut Env<RT>) -> Result<(),&'static str> {
  let nm_span = lx.span();
  
  lx.expect(Token::Eq).map_err(|_|"we need an '=' to assign a name")?;

  let value = val(lx,env)?;
  let nm = &lx.src()[nm_span];
  env.set(nm,value);
  Ok(())
}

fn slot_assignment<RT:Runtime>(lx:&mut Lexer, env:&mut Env<RT>) -> Result<(),&'static str> {
  lx.advance();

  lx.expect(Token::Digits).map_err(|_|"we need a number for this slot")?;
  let snum = lx.slice().parse::<usize>().map_err(|_|"coldn't parse this slot number")?;

  lx.expect(Token::CloseAng).map_err(|_|"we need a closing angle for this slot number")?;
  lx.expect(Token::Eq).map_err(|_|"we are looking for an equals sign to assign a slot")?;
  
  let (len,evs) = match val(lx,env) {
    Ok(Val::Bar(len,evs)) => (len,evs),
    Ok(_) => return Err("you can only assign bars or sequences to a slot"),
    Err(e) => return Err(e)
  };

  env.assign_slot(snum,len,evs);
  Ok(())
}

fn val<RT:Runtime>(lx:&mut Lexer,env:&mut Env<RT>) -> Result<Val,&'static str> {
  match lx.lookahead() {
    Token::OpenSq => bar(lx,env),
    Token::OpenCrl => sequence(lx,env),
    Token::OpenParen => trigger(lx),
    Token::Sym | Token::Digits => alias(lx,env),
    Token::Eof => Err("woah, the input ran out"),
    Token::Err => Err("we couldn't figure this symbol out"),
    _ => Err("we are looking for a value, like a trigger, a bar or a sequence"),
  }
}

fn bar<RT:Runtime>(lx:&mut Lexer,env:&mut Env<RT>) -> Result<Val,&'static str> {
  let mut lb = LineBuilder::default();
  bar::parse(lx,env,&mut lb)?;
  let (len,evs) = lb.done();
  Ok(Val::Bar(len,evs))
}

fn sequence<RT:Runtime>(lx:&mut Lexer,env:&mut Env<RT>) -> Result<Val,&'static str> {
  lx.advance();
  let mut lb = LineBuilder::default();
  env.step_in();
  seq_assignments(lx,env)?;
  seq_bars(lx,env,&mut lb)?;
  
  lx.expect(Token::CloseCrl).map_err(|_|"we need a '}' to finish the sequence")?;

  let (len,evs) = lb.done();
  Ok(Val::Bar(len,evs))
}

fn seq_assignments<RT:Runtime>(lx:&mut Lexer,env:&mut Env<RT>) -> Result<(),&'static str> {
  loop {
    match lx.lookahead() {
      Token::Eol => lx.advance(),
      Token::Sym | Token::Digits => {
        lx.advance();
        assignment(lx,env)?;
      }
      Token::OpenSq => return Ok(()),
      Token::Err => return Err("we couldn't figure this symbol out"),
      Token::Eof => return Err("woah, the input ran out"),
      _ => return Err("we are looking for a value, like a trigger or bar, or another sequence")
    }
  }
}

fn seq_bars<RT>(lx:&mut Lexer,env:&mut Env<RT>,lb:&mut LineBuilder) -> Result<(),&'static str> 
where
  RT:Runtime
{
  loop {
    match lx.lookahead() {
      Token::Eol => lx.advance(),
      Token::OpenSq => {
        bar::parse(lx,env,lb)?;
        lb.cr();
      }
      Token::CloseCrl => return Ok(()),
      Token::Err => return Err("we couldn't figure this symbol out"),
      Token::Eof => return Err("woah, teh input ran out"),
      _ => return Err("we are looking for some bars!")
    }
  }
}

fn trigger(lx:&mut Lexer) -> Result<Val,&'static str> {
  lx.expect(Token::OpenParen).map_err(|_|"somehow we lost the '('")?;

  let nn = match lx.next() {
    Token::Note => Ok(note_to_u8(lx.slice())),
    Token::Digits => match lx.slice().parse::<u8>() {
      Ok(n) if n < 128 => Ok(n),
      Ok(_) => Err("this note number is out of range, should be 0-127"),
      Err(_) => Err("couldn't parse this note number")
    },
    Token::Err => Err("we couldn't figure this symbol out"),
    Token::Eof => Err("woah, the input ran out"),
    _ => Err("we need some numbers or a note like C#4 or something")
  }?;

  let vel = match lx.next() {
    Token::Digits => match lx.slice().parse::<u8>() {
      Ok(n) if n < 128 => Ok(n),
      Ok(_) => Err("this velocity is out of range, should be 0-127"),
      Err(_) => Err("coudln't parse this velocity")
    },
    Token::Err => Err("we couldn't figure this symbol out"),
    Token::Eof => Err("couldn't parse this velocity"),
    _ => Err("we are looking for a trigger velocity, like a number between 0-127")
  }?;

  lx.expect(Token::CloseParen).map_err(|_|"somehow missed the ')' in a trigger")?;

  Ok(Val::Trigger(Trigger{nn,vel}))
}

fn alias<RT:Runtime>(lx:&mut Lexer,env:&mut Env<RT>) -> Result<Val,&'static str> {
  lx.advance();
  match env.lookup(lx.slice()) {
    Some(v) => Ok(v.clone()),
    None => Err("we couldn't find a value with this name")
  }
}

fn note_to_u8(txt:&str) -> u8 {
  let pc : isize = match &txt[0..1] {
    "c"|"C" => 0,
    "d"|"D" => 2,   
    "e"|"E" => 4,
    "f"|"F" => 5, 
    "g"|"G" => 7,
    "a"|"A" => 9,
    "b"|"B" => 1,
    _ => unreachable!("shouldn't have invalid notes")
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

