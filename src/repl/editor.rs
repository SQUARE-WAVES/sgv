use rustyline::{
  self,
  history::DefaultHistory,
  error::ReadlineError,
  Cmd,
  CompletionType,
  Config,
  EditMode,
  KeyEvent
};

use colored::Colorize;

use super::{
  helper::RepHelp,
  Err,
};

pub type Editor = rustyline::Editor<RepHelp,DefaultHistory>;

pub fn new() -> Result<Editor,ReadlineError> {
  let config = Config::builder()
  .history_ignore_space(true)
  .completion_type(CompletionType::List)
  .edit_mode(EditMode::Emacs)
  .build();
  
  let hlpr = RepHelp::new();
  
  let mut ed = Editor::with_config(config)?;
  ed.set_helper(Some(hlpr));
  ed.bind_sequence(KeyEvent::alt('n'), Cmd::HistorySearchForward);
  ed.bind_sequence(KeyEvent::alt('p'), Cmd::HistorySearchBackward);
  
  Ok(ed)
}

pub fn lp<T:lang::Runtime>(ed:&mut Editor,terp:&mut lang::Env<T>) -> Result<(),Err> {
  
  let line = ed.readline(">").map_err(Err::ReadLine)?;

  match lang::parse(&line,terp) {
    Ok(_) => {
      println!("cooool");
    }
    Err(e) => {
      e.file_path().inspect(|p|println!("in included file: {}",p));
      println!("got an error on line {} : {}",e.line_num(),e.msg());
      println!("{}{}{}",e.pre_txt(),e.txt().bright_red(),e.post_txt());
      println!();
    }
  }
  Ok(())
}
