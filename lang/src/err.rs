pub enum ParseError {
  Msg(&'static str),
  SubFile(&'static str,String,usize,usize)
}

impl From<&'static str> for ParseError {
  fn from(msg:&'static str) -> Self {
    Self::Msg(msg)
  }
}

pub enum Error<'a> {
  Root(&'a str,usize,usize,&'static str),
  Sub(&'a str,String,usize,usize,&'static str)
}

impl Error<'_> {
  pub fn file_path(&self) -> Option<&str> {
    match self {
      Self::Sub(p,_,_,_,_) => Some(p),
      _ => None
    }
  }

  pub fn line_num(&self) -> usize {
    let (src,s) = match self {
      Self::Root(src,s,_,_) => (*src,*s),
      Self::Sub(_,src,s,_,_) => (&src[..],*s)
    };

    src[..s].lines().count()
  }

  pub fn msg(&self) -> &'static str {
    match self {
      Self::Root(_,_,_,msg) => msg,
      Self::Sub(_,_,_,_,msg) => msg
    }
  }

  pub fn pre_txt(&self) -> &str {
    let (src,s) = match self {
      Self::Root(src,s,_,_) => (*src,*s),
      Self::Sub(_,src,s,_,_) => (&src[..],*s)
    };

    let ln_start = src[..s].rfind('\n').map(|n|n+1).unwrap_or(0);
    &src[ln_start..s]
  }

  pub fn txt(&self) -> &str {
    match self {
      Self::Root(src,s,e,_) => &src[*s..*e],
      Self::Sub(_,src,s,e,_) => &src[*s..*e]
    }
  }

  pub fn post_txt(&self) -> &str {
    let (src,e) = match self {
      Self::Root(src,_,e,_) => (*src,*e),
      Self::Sub(_,src,_,e,_) => (&src[..],*e)
    };

    let ln_end = src[e..].find('\n').map(|n|n+e).unwrap_or(src.len());
    &src[e..ln_end]
  }
}
