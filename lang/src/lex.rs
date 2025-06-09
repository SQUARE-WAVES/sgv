use logos::Logos;

#[derive(Logos,Copy,Clone,Debug,PartialEq,Eq)]
#[logos(skip r"[ \t\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
  //KEYWORDZ
  #[token("bpm",ignore(case))]
  Bpm,

  #[token("div",ignore(case))]
  Div,
  
  #[token("sync",ignore(case))]
  Sync,

  #[token("play",ignore(case))]
  Play,

  #[token("stop",ignore(case))]
  Stop,

  #[token("list_outs",ignore(case))]
  ListOuts,

  #[token("open_output",ignore(case))]
  OpenOut,

  #[token("set_output",ignore(case))]
  SetOut,

  #[token("find_lps",ignore(case))]
  FindLps,

  #[token("open_lp",ignore(case))]
  OpenLp,

  #[token("do_file",ignore(case))]
  DoFile,

  //--------

  #[token("(")]
  OpenParen,

  #[token(")")]
  CloseParen,

  #[token("[")]
  OpenSq,

  #[token("]")]
  CloseSq,

  #[token("{")]
  OpenCrl,

  #[token("}")]
  CloseCrl,

  #[token("<")]
  OpenAng,

  #[token(">")]
  CloseAng,
  
  #[token("=")]
  Eq,

  #[token("-")]
  Dash,
  
  #[token("\n")]
  Eol,

  #[regex(r"[abcdefgABCDEFG][\_#b][0-8]")]
  Note,

  #[regex(r"\d+", priority=10)]
  Digits,

  #[regex(r"[\d\w]+", priority=5)]
  Sym,

  //these are things the lexer will never
  //match
  Err,
  Eof
}

pub enum ExpectErr {
  Eof,
  LexErr,
  WrongToken(Token)
}


#[derive(Debug)]
pub struct Lexer<'a> {
  lx:logos::Lexer<'a,Token>,
  cspan:std::ops::Range<usize>,
  current:Token
}

impl<'a> Lexer<'a> {
  pub fn new(src:&'a str) -> Self {
    let mut lx = Token::lexer(src);
    let sp = lx.span();

    let first = match lx.next() {
      Some(Ok(t)) => t,
      Some(Err(_)) => Token::Err,
      None => Token::Eof
    };

    Self {
      lx,
      cspan:sp,
      current:first
    }
  }

  pub fn lookahead(&self) -> Token {
    self.current
  }

  pub fn advance(&mut self) {
    let _ = self.next();
  }

  pub fn span(&self) -> std::ops::Range<usize> {
    self.cspan.clone()
  }

  pub fn span_start(&self) -> usize {
    self.cspan.start
  }

  pub fn span_end(&self) -> usize {
    self.cspan.end
  }

  pub fn slice(&self) -> &str {
    &self.lx.source()[self.cspan.clone()]
  }

  pub fn src(&self) -> &str {
    self.lx.source()
  }

  pub fn next(&mut self) -> Token {
    let out = self.current;
    self.cspan = self.lx.span();

    self.current = match self.lx.next() {
      Some(Ok(t)) => t,
      Some(Err(_)) => Token::Err,
      None => Token::Eof
    };
    out
  }

  pub fn expect(&mut self,tk:Token) -> Result<(),ExpectErr>{
    match self.next() {
      t if t == tk => Ok(()),
      Token::Err => Err(ExpectErr::LexErr),
      Token::Eof => Err(ExpectErr::Eof),
      t => Err(ExpectErr::WrongToken(t)),
    }
  }

  pub fn done(self) -> (&'a str,usize,usize) {
    let src = self.lx.source();
    let start = self.span_start();
    let end = self.span_end();
    (src,start,end)
  }
}

impl Token {
  pub fn parse(self, mut lx:Lexer) -> Result<((Token,std::ops::Range<usize>),Lexer),Lexer> {
    match lx.next() {
      t if t == self => Ok(( (t,lx.span()),lx )),
      _ => Err(lx)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_lexer() {
    let input = "sym [ ] doookz > #";
    let mut mylex = Lexer::new(input);
    let mut lx = Token::lexer(input);

    assert_eq!(mylex.span(),lx.span());
    assert_eq!(mylex.slice(),lx.slice());
    assert_eq!(mylex.lookahead(),lx.next().unwrap().unwrap());
    mylex.advance();

    assert_eq!(mylex.span(),lx.span());
    assert_eq!(mylex.slice(),lx.slice());
    assert_eq!(mylex.lookahead(),lx.next().unwrap().unwrap());
    mylex.advance();

    assert_eq!(mylex.span(),lx.span());
    assert_eq!(mylex.slice(),lx.slice());
    assert_eq!(mylex.lookahead(),lx.next().unwrap().unwrap());
    mylex.advance();

    assert_eq!(mylex.span(),lx.span());
    assert_eq!(mylex.slice(),lx.slice());
    assert_eq!(mylex.lookahead(),lx.next().unwrap().unwrap());
    mylex.advance();

    assert_eq!(mylex.span(),lx.span());
    assert_eq!(mylex.slice(),lx.slice());
    assert_eq!(mylex.lookahead(),lx.next().unwrap().unwrap());
    mylex.advance();

    assert_eq!(mylex.span(),lx.span());
    assert_eq!(mylex.slice(),lx.slice());
    assert_eq!(mylex.lookahead(),Token::Err);
    assert!(matches!(lx.next(),Some(Err(()))));
    mylex.advance();

    assert_eq!(mylex.span(),lx.span());
    assert_eq!(mylex.slice(),lx.slice());
    assert_eq!(mylex.lookahead(),Token::Eof);
    assert!(lx.next().is_none());
    mylex.advance();

    //assert the lookahead works correctly
    let mut lx = Lexer::new(input);
    assert_eq!(lx.lookahead(),lx.next());
    assert_eq!(lx.lookahead(),lx.next());
    assert_eq!(lx.lookahead(),lx.next());
    assert_eq!(lx.lookahead(),lx.next());
    assert_eq!(lx.lookahead(),lx.next());
    assert_eq!(lx.lookahead(),lx.next());
  }

  #[test]
  fn test_tokens() {
    let input = "BpM";
    let mut lxr = Token::lexer(input);
    assert!(matches!(lxr.next(),Some(Ok(Token::Bpm))));

    let input = "/fish/wish";
    let mut lxr = Token::lexer(input);
    let nxt = lxr.next();
    assert!(matches!(nxt,Some(Ok(Token::Path))));
  }
}
