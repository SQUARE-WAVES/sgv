use super::{Lexer,Token,Env,Runtime};
use crate::seq_types::{Val,Trigger,LineBuilder};

pub fn parse<RT>(lx:&mut Lexer,env:&Env<RT>,line:&mut LineBuilder) -> Result<(),&'static str>
where
  RT:Runtime
{
  lx.expect(Token::OpenSq).map_err(|_|"somehow missed the '['")?;

  loop {
    match lx.lookahead() {
      Token::CloseSq => {
        lx.advance();
        break;
      },
      Token::Dash => bar_rests(lx,line),
      Token::Sym | Token::Digits => bar_trigger(lx,env,line),
      Token::Err => return Err("we couldn't figure this symbol out"),
      Token::Eof => return Err("woah, the input ended"),
      _ => {
        //you gotta step forward or else you will highlight the last good token
        lx.advance();
        return Err("we are working on a bar, meaning we need a name or a - or ]")
      }
    }?;
  };

  Ok(())
}

fn bar_rests(lx:&mut Lexer,line:&mut LineBuilder) -> Result<(),&'static str> {
  lx.expect(Token::Dash).map_err(|_|"somehow missed the first '-'")?;
  let mut count = 1;

  while lx.lookahead() == Token::Dash {
    count += 1;
    lx.advance();
  }

  line.rests(count);
  Ok(())
}

fn bar_trigger<RT>(lx:&mut Lexer,env:&Env<RT>,line:&mut LineBuilder) -> Result<(),&'static str> 
where
  RT:Runtime
{
  lx.advance();
  let nm = lx.slice();

  let val = match env.lookup(nm) {
    Some(v) => v,
    None => return Err("we couldn't find a value with this name")
  };
  
  let mut ties = 0;
  while lx.lookahead() == Token::Eq {
    ties += 1;
    lx.advance();
  }

  let legato = if lx.lookahead() == Token::CloseAng {
    lx.advance();
    true
  }
  else {
    false
  };

  match val {
    Val::Trigger(Trigger{nn,vel}) => line.trig(*nn,*vel,ties,legato),
    Val::Bar(len,evs) => {
      line.merge(*len,evs);
      line.rests(ties);
    }
  };

  Ok(())
}

#[cfg(test)]
mod tests{
  use crate::test_utils;
  use super::*;

  #[test]
  fn test_bar() {
    let mut env = Env::new(test_utils::NullRt{});
    env.set("X",Val::Trigger(Trigger{nn:1,vel:1}));

    let mut lb = LineBuilder::default();
    let input = "[X - - -]";
    let mut lx = Lexer::new(input);

    parse(&mut lx,&env,&mut lb).expect("shouldn't error");
    let (len,evs) = lb.done();
    assert_eq!(len,4);
    assert_eq!(evs.len(),1);
    assert_eq!(evs.get(&0).unwrap().len(),1);
    
    env.set("bounce",Val::Bar(len,evs));
    let mut lb = LineBuilder::default();
    let input = "[bounce bounce bounce bounce]";
    let mut lx = Lexer::new(input);
    parse(&mut lx,&env,&mut lb).expect("shouldn't error");
    let (len,evs) = lb.done();
    assert_eq!(len,16);
    assert_eq!(evs.len(),4);
    assert_eq!(evs.get(&0).unwrap().len(),1);
    assert_eq!(evs.get(&4).unwrap().len(),1);
    assert_eq!(evs.get(&8).unwrap().len(),1);
    assert_eq!(evs.get(&12).unwrap().len(),1);

    //TODO::TEST ERROR CASES
  }
}
