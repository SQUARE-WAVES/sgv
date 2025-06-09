use std::borrow::Cow::{self, Borrowed, Owned};
use colored::Colorize;

use rustyline::{
  Helper,
  Completer,
  Hinter,
  Validator,
  highlight::{
    CmdKind, 
    Highlighter, 
    MatchingBracketHighlighter
  },
  hint::HistoryHinter,
  validate::MatchingBracketValidator,
  completion::FilenameCompleter,
};


#[derive(Helper, Completer, Hinter, Validator)]
pub struct RepHelp {
  #[rustyline(Completer)]
  completer: FilenameCompleter,
  #[rustyline(Validator)]
  validator: MatchingBracketValidator,
  #[rustyline(Hinter)]
  hinter: HistoryHinter,
  highlighter: MatchingBracketHighlighter,
}

impl Highlighter for RepHelp {
  fn highlight_prompt<'b, 's: 'b, 'p: 'b>(&'s self,pmpt: &'p str,default: bool) -> Cow<'b, str> {
    if default {
      Borrowed(">")
    } else {
      Borrowed(pmpt)
    }
  }
  
  fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
    let outz = format!("{}{}{}","[".bright_blue(),hint,"]".bold());
    Owned(outz)
  }

  fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
    self.highlighter.highlight(line, pos)
  }

  fn highlight_char(&self, line: &str, pos: usize, kind: CmdKind) -> bool {
    self.highlighter.highlight_char(line, pos, kind)
  }
}

impl RepHelp {
  pub fn new() -> Self {
    Self {    
      completer: FilenameCompleter::new(),
      highlighter: MatchingBracketHighlighter::new(),
      hinter: HistoryHinter::new(),
      validator: MatchingBracketValidator::new(),
    }
  }
}


