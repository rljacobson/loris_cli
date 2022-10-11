/*!

A minimal REPL executable.

 */

use rustyline::error::ReadlineError;
use rustyline::{Editor, Result as RLResult};

use lorislib::{
  Parser,
  Context,
  logging::set_verbosity,
  evaluate,
};


struct Session {
  parser: Parser,
  context: Context
}

impl Default for Session {
  fn default() -> Session {
    Session {
      parser: Parser::new(),
      context: Context::new_global_context()
    }
  }
}

impl Session {
  fn new() -> Session {
    Self::default()
  }

  fn process_input(&mut self, input: &str) -> Result<(), String> {
    let result = self.parser.parse(input);
    match result {

      Ok(expression) => {
        let result = evaluate(expression, &mut self.context);
        println!("{}\n", result);

        Ok(())
      }

      _ => {
        Err("Failed to parse.\n".to_string())
      }

    }
  }
}



fn main() -> RLResult<()> {
  println!("\nLoris term rewriting system version 0.1.0.\n\n");
  // set_verbosity(5);
  let mut session = Session::new();

  // Todo: replace `()` with completer.
  let mut rl = Editor::<()>::new()?;
  /*
  if rl.load_history("history.txt").is_err() {
    println!("No previous history.");
  }
  */

  loop {
    let readline = rl.readline(":> "); // Prompt doesn't work within IntelliJ.
    match readline {
      Ok(line) => {
        // rl.add_history_entry(line.as_str());

        // Check for meta commands.
        match line.as_str() {
          "end"
          | "exit" => break,

          _ => {
            /* pass */
          }
        }

        // todo: Do we need a `Result` to be returned to the REPL?
        match session.process_input(line.as_str()) {
          Ok(()) => {}
          Err(msg) => {
            println!("{}", msg);
          }
        }

      },
      Err(ReadlineError::Interrupted) => {
        println!("CTRL-C");
        break
      },
      Err(ReadlineError::Eof) => {
        println!("CTRL-D");
        break
      },
      Err(err) => {
        println!("Error: {:?}\n", err);
      }
    } // end match readline
  } // end loop

  // rl.save_history("history.txt")
  Ok(())
}




#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
