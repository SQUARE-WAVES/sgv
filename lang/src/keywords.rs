
use super::{Lexer,Token,Runtime,Env,err};

pub fn bpm<RT:Runtime>(lx:&mut Lexer,env:&mut Env<RT>) -> Result<(),&'static str> {
  lx.advance();

  lx.expect(Token::Digits).map_err(|_|"expected a bpm value, like 120")?;
  let bpm_num= lx.slice().parse::<usize>().map_err(|_|"couldn't parse the bpm number")?;

  env.set_bpm(bpm_num);
  Ok(())
}

pub fn div<RT:Runtime>(lx:&mut Lexer,env:&mut Env<RT>) -> Result<(),&'static str> {
  lx.advance();

  lx.expect(Token::Digits).map_err(|_|"we need a slot number")?;
  let slot_num= lx.slice().parse::<usize>().map_err(|_|"couldn't parse this slot number")?;
  lx.expect(Token::Digits).map_err(|_|"we need a time division")?;
  let div_num= lx.slice().parse::<usize>().map_err(|_|"couldn't parse this time division")?;

  env.set_div(slot_num,div_num);
  Ok(())
}

pub fn sync<RT:Runtime>(lx:&mut Lexer,env:&mut Env<RT>) -> Result<(),&'static str> {
  lx.advance();

  lx.expect(Token::Digits).map_err(|_|"we need a slot number")?;
  let slot_num= lx.slice().parse::<usize>().map_err(|_|"couldn't parse this slot number")?;
  lx.expect(Token::Digits).map_err(|_|"we need a sync division")?;
  let sync_num= lx.slice().parse::<usize>().map_err(|_|"couldn't parse the sync division")?;

  env.set_sync(slot_num,sync_num);
  Ok(())
}

pub fn play<RT:Runtime>(lx:&mut Lexer,env:&mut Env<RT>) -> Result<(),&'static str> {
  lx.advance();

  lx.expect(Token::Digits).map_err(|_|"we need a slot number")?;
  let slot_num = lx.slice().parse::<usize>().map_err(|_|"couldn't parse this slot number")?;

  env.play_slot(slot_num);
  Ok(())
}

pub fn stop<RT:Runtime>(lx:&mut Lexer,env:&mut Env<RT>) -> Result<(),&'static str> {
  lx.advance();

  lx.expect(Token::Digits).map_err(|_|"we need a slot number")?;
  let slot_num = lx.slice().parse::<usize>().map_err(|_|"couldn't parse this slot number")?;

  env.stop_slot(slot_num);
  Ok(())
}

pub fn list_outs<RT:Runtime>(lx:&mut Lexer,env:&mut Env<RT>) -> Result<(),&'static str> {
  lx.advance();
  env.list_outs();
  Ok(())
}

pub fn open_out<RT:Runtime>(lx:&mut Lexer,env:&mut Env<RT>) -> Result<(),&'static str> {
  lx.advance();

  lx.expect(Token::Digits).map_err(|_|"we need an output number")?;
  let out_num = lx.slice().parse::<usize>().map_err(|_|"couldn't parse this output num")?;

  lx.expect(Token::Digits).map_err(|_|"we need a channel number")?;
  let chan = lx.slice().parse::<u8>().map_err(|_|"couldn't parse this channel num")?;
  if chan > 15 {
    return Err("channel is out of range, must be 0 - 15");
  }

  env.open_out(out_num,chan);
  Ok(())
}

pub fn set_out<RT:Runtime>(lx:&mut Lexer,env:&mut Env<RT>) -> Result<(),&'static str> {
  lx.advance();

  lx.expect(Token::Digits).map_err(|_|"we need a slot number")?;
  let slot_num= lx.slice().parse::<usize>().map_err(|_|"couldn't parse this slot number")?;
  lx.expect(Token::Digits).map_err(|_|"we need an out_number")?;
  let sync_num= lx.slice().parse::<usize>().map_err(|_|"couldn't parse the out numver")?;

  env.set_output(slot_num,sync_num);
  Ok(())
}

pub fn list_lps<RT:Runtime>(lx:&mut Lexer,env:&mut Env<RT>) -> Result<(),&'static str> {
  lx.advance();
  env.list_lps();
  Ok(())
}

pub fn open_lp<RT:Runtime>(lx:&mut Lexer,env:&mut Env<RT>) -> Result<(),&'static str> {
  lx.advance();

  lx.expect(Token::Digits).map_err(|_|"we need a launchpad number")?;
  let lp_num = lx.slice().parse::<usize>().map_err(|_|"couldn't parse this launchpad number")?;
  env.open_lp(lp_num);
  Ok(())
}

pub fn do_file<RT:Runtime>(lx:&mut Lexer,env:&mut Env<RT>) -> Result<(),err::ParseError> {
  lx.advance();

  let start = match lx.lookahead() {
    Token::Eof | Token::Eol => return Err(err::ParseError::Msg("we need a path to the file")),
    _ => {
      lx.advance();
      lx.span_start()
    }
  };

  let end = loop {
    lx.advance();
    match lx.lookahead() {
      Token::Eof | Token::Eol => {
        break lx.span_end();
      }
      _ => ()
    }
  };

  lx.advance();

  env.do_file(&lx.src()[start..end])?;
  Ok(())
}

