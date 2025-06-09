use pekoms::Parser;

use super::{
  Token,
  Lexer,
  Env,
  Val,
  err,
  trigger,
  bar,
  sequence
};

pub fn asgn<'a>(env:&mut Env) -> impl FnOnce(Lexer<'a>) -> Result<((),Lexer<'a>),err::Error<'a>> {
  |lx| {
    let (r,lx) = name.parse(lx)?;
    let (_,mut lx) = Token::Eq.map_err(|lx2|err::asign(lx2,ErrCause::Eq)).parse(lx)?;

    match lx.lookahead() {
      Some(Ok(Token::Sym)) | Some(Ok(Token::Digits)) => {
        lx.advance();
        let name = lx.slice();
        let alias = &lx.src()[r];
        match env.lookup(name) {
          Some(val) => { 
            env.set(alias,val.clone());
            Ok(((),lx))
          }
          None => Err(err::asign(lx,ErrCause::UnknownVar))
        }
      },
      Some(Ok(Token::OpenSq)) => {
        let (seq,rest) = bar::barc(env).parse(lx)?;
        let name = &rest.src()[r];
        env.set(name,seq);
        Ok(((),rest))
      },
      Some(Ok(Token::OpenCrl)) => {
        let (seq,rest) = sequence::seqc(env).parse(lx)?;
        let name = &rest.src()[r];
        env.set(name,seq);
        Ok(((),rest))
      },
      Some(Ok(Token::OpenParen)) => {
        let (trig,rest) = trigger::trigger.parse(lx)?;
        let name = &rest.src()[r];
        env.set(name,Val::Trigger(trig.note.val,trig.vel.val));

        Ok(((),rest))
      }
      Some(Ok(_)) => Err(err::asign(lx,ErrCause::Var)),
      Some(Err(_)) => Err(err::lex(lx)),
      None => Err(err::eof(lx))
    }
  }
}

fn name(mut lx:Lexer) -> Result<(std::ops::Range<usize>,Lexer),err::Error> {
  match lx.next() {
    Some(Ok(Token::Sym)) | Some(Ok(Token::Digits)) => Ok( (lx.span(),lx) ),
    Some(Ok(_)) => Err(err::asign(lx,ErrCause::VarName)),
    Some (Err(_))=> Err(err::lex(lx)),
    None => Err(err::eof(lx)),
  }
}

#[derive(Debug,PartialEq,Eq)]
pub enum ErrCause {
  Var,
  VarName,
  UnknownVar,
  Eq,
}
