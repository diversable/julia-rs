
//! Minimalistic interactive Julia REPL written in Rust.
//!
//! ## Compilation
//! `cargo make`
//!
//! ## Usage
//!
//! `cargo run`
//! OR
//! `julia-rs`
//! if installed with `cargo install`
//!
//! ## Example
//!
//! ```julia
//!
//!                _
//!    _       _ _(_)_                   |  A fresh approach to technical computing
//!   (_)     | (_) (_)                  |  Documentation: https://docs.julialang.org
//!    _ _   _| |_  __ _     _  _  __    |  Rusty REPL based on official Julia REPL
//!   | | | | | | |/ _` |   | |/ // _)   |
//!   | | |_| | | | (_| | _ |  ,/ \_ \   |  julia 0.6.0 (9036443)
//!  _/ |\__'_|_|_|\__'_|(_)|_|   (__/   | julia-rs 0.1.2 (master-6a5b7d1)
//! |__/                                 |
//!
//! julia.rs> f(x) = 2x + pi
//! ```

#![feature(unicode)]

extern crate julia;
extern crate colored;
extern crate liner;
extern crate clap;
extern crate std_unicode;

use std::env;
use std::fs::File;
use std::io::ErrorKind;
use std_unicode::str::UnicodeStr;

use liner::{Context, History, KeyBindings};
use colored::*;
use clap::{Arg, App};

use julia::api::{Julia, Ref};
use julia::error::Error;
use julia::version;

const INITREPL_JL: &str = "isinteractive() = true
exit() = println(\"Sorry! Use C-D to exit.\")
exit(s) = exit()";

macro_rules! errprintln {
    ($msg:expr) => { eprintln!("{}", msg.bright_red().bold()); };
    ($fmt:expr, $err:expr) => {
        {
            use std::fmt::Write;
            use std::error::Error;

            let mut msg = String::new();
            let err = match $err.cause() {
                None        => {
                    write!(msg, concat!($fmt, "\n > {}"), $err, $err.description())
                        .and_then(|_| {
                            eprintln!("{}", msg.bright_red().bold());
                            Ok(())
                        })
                },
                Some(cause) => {
                    write!(msg, concat!($fmt, "\n > {}\n >> {}"), $err, $err.description(), cause)
                        .and_then(|_| {
                            eprintln!("{}", msg.bright_red().bold());
                            Ok(())
                        })
                },
            };
            err.expect("Couldn't write error");
        }
    }
}

fn greet(jl: &Julia) {
    println!(
        r#"               {}
   {}       _ {}{}{}                   |  A fresh approach to technical computing
  {}     | {} {}                  |  Documentation: https://docs.julialang.org
   _ _   _| |_  __ _     _  _  __    |  Rusty REPL based on official Julia REPL
  | | | | | | |/ _` |   | |/ // _)   |
  | | |_| | | | (_| | {} |  ,/ \_ \   |  {}
 _/ |\__'_|_|_|\__'_|{}|_|   (__/   |  {}
|__/                                 |
"#,
        "_".bright_green().bold(),
        "_".bright_blue().bold(),
        "_".bright_red().bold(),
        "(_)".bright_green().bold(),
        "_".bright_magenta().bold(),
        "(_)".bright_blue().bold(),
        "(_)".bright_red().bold(),
        "(_)".bright_magenta().bold(),
        "_".bright_yellow().bold(),
        jl.version(),
        "(_)".bright_yellow().bold(),
        version::get()
    );
}

fn set_history(jl: &mut Julia, ret: &Ref) -> Result<(), usize> {
    let ans = jl.main().global("ans").unwrap_or_else(|_| Ref::nothing());
    let ans1 = jl.main().global("ans1").unwrap_or_else(|_| Ref::nothing());
    let ans2 = jl.main().global("ans2").unwrap_or_else(|_| Ref::nothing());
    let ans3 = jl.main().global("ans3").unwrap_or_else(|_| Ref::nothing());
    let ans4 = jl.main().global("ans4").unwrap_or_else(|_| Ref::nothing());
    let ans5 = jl.main().global("ans5").unwrap_or_else(|_| Ref::nothing());
    let ans6 = jl.main().global("ans6").unwrap_or_else(|_| Ref::nothing());
    let ans7 = jl.main().global("ans7").unwrap_or_else(|_| Ref::nothing());
    let ans8 = jl.main().global("ans8").unwrap_or_else(|_| Ref::nothing());
    jl.main().set("ans", ret).map_err(|_| 0_usize)?;
    jl.main().set("ans1", &ans).map_err(|_| 1_usize)?;
    jl.main().set("ans2", &ans1).map_err(|_| 2_usize)?;
    jl.main().set("ans3", &ans2).map_err(|_| 3_usize)?;
    jl.main().set("ans4", &ans3).map_err(|_| 4_usize)?;
    jl.main().set("ans5", &ans4).map_err(|_| 5_usize)?;
    jl.main().set("ans6", &ans5).map_err(|_| 6_usize)?;
    jl.main().set("ans7", &ans6).map_err(|_| 7_usize)?;
    jl.main().set("ans8", &ans7).map_err(|_| 8_usize)?;
    jl.main().set("ans9", &ans8).map_err(|_| 9_usize)?;
    Ok(())
}

fn eval_string(jl: &mut Julia, expr: &str) -> Option<Ref> {
    let ret = jl.eval_string(expr);

    let ret = match ret {
        Ok(ret) => ret,
        Err(Error::UnhandledException(ex)) => {
            errprintln!("Exception: {}", ex);
            return None;
        }
        Err(err) => {
            errprintln!("Error: {}", err);
            return None;
        }
    };

    if !ret.is_nothing() { Some(ret) } else { None }
}

fn interactive(mut jl: Julia, quiet: bool) {
    if !quiet {
        greet(&jl);
    }

    let home = env::var("HOME").unwrap();
    let history_path = format!("{}/.julia-rs_history", home);
    let mut history = History::new();

    history.set_file_name(Some(history_path));
    history.load_history().ok();

    let mut con = Context {
        history: history,
        completer: None,
        word_divider_fn: Box::new(liner::get_buffer_words),
        key_bindings: KeyBindings::Emacs,
    };
    let ps1 = format!("{} ", "julia.rs>".bright_green().bold());

    loop {
        let line = con.read_line(&*ps1, &mut |_| {});
        let line = match line {
            Ok(ref line) if line.is_empty() || line.is_whitespace() => continue,
            Ok(line) => line,
            Err(err) => {
                match err.kind() {
                    ErrorKind::Interrupted => continue,
                    ErrorKind::UnexpectedEof => break,
                    err => {
                        eprintln!("Error: {:?}", err);
                        continue;
                    }
                }
            }
        };

        let ret = eval_string(&mut jl, &*line);
        if let Some(ret) = ret {
            print!("{}", ret);

            if let Err(i) = set_history(&mut jl, &ret) {
                eprintln!("Warning: couldn't set answer history at {}", i);
            }
        }
        println!();

        if let Err(err) = con.history.push(line.into()) {
            eprintln!("Error: could not write line to history file\n > {}", err);
        }
    }

    let Context { mut history, .. } = con;
    history.commit_history();
}

fn main() {
    let ver = version::get().to_string();
    let app = App::new("")
        .version(&*ver)
        .author("Szymon Walter <walter.szymon.98@gmail.com>")
        .about("Minimalistic interactive Julia REPL in Rust")
        .arg(
            Arg::with_name("eval")
                .short("e")
                .long("eval")
                .value_name("EXPR")
                .help("Evaluate EXPR")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("print")
                .short("E")
                .long("print")
                .value_name("EXPR")
                .help("Evaluate and show EXPR")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("load")
                .short("L")
                .long("load")
                .value_name("FILE")
                .help("Load FILE")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dlpath")
                .short("S")
                .long("so")
                .value_name("PATH")
                .help("Search for shared objects in PATH")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dlopen")
                .short("l")
                .long("lib")
                .value_name("LIB")
                .help("Open LIB as a shared object")
                .multiple(true)
                .takes_value(true),
        )
        .arg(Arg::with_name("repl").short("i").long("interactive").help(
            "Interactive mode; REPL runs and isinteractive() is true",
        ))
        .arg(Arg::with_name("quiet").short("q").long("quiet").help(
            "Quiet startup (no banner)",
        ));

    let matches = app.get_matches();

    let eval = matches.values_of("eval");
    let print = matches.values_of("print");
    let load = matches.values_of("load");
    let dlpath = matches.values_of("dlpath");
    let dlopen = matches.values_of("dlopen");
    let repl = matches.is_present("repl");
    let quiet = matches.is_present("quiet");

    let mut jl = Julia::new();

    let mut repl_default = true;

    let mut dlinclude = String::new();
    if let Some(paths) = dlpath {
        for path in paths {
            dlinclude.push_str(&format!("push!(Libdl.DL_LOAD_PATH, \"{}\")\n", path));
        }
    }

    if let Some(paths) = dlopen {
        for path in paths {
            dlinclude.push_str(&format!(
                "{} = Libdl.dlopen(\"{}\")",
                path.replace(".so", ""),
                path
            ));
        }
    }

    if !dlinclude.is_empty() {
        jl.load(&mut dlinclude.as_bytes(), Some("dlinclude.jl"))
            .expect("Could not load dlinclude.jl");
    }

    if let Some(eval) = eval {
        for expr in eval {
            eval_string(&mut jl, expr);
        }
        repl_default = false;
    }

    if let Some(print) = print {
        for expr in print {
            if let Some(string) = eval_string(&mut jl, expr) {
                println!("{}", string);
            }
        }
        repl_default = false;
    }

    if let Some(load) = load {
        for filename in load {
            let mut file = match File::open(filename) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error: couldn't open file\n > {}", e);
                    continue;
                }
            };

            match jl.load(&mut file, Some(filename)) {
                Err(Error::UnhandledException(ex)) => errprintln!("Exception: {}", ex),
                Err(err) => errprintln!("Error: {}", err),
                _ => (),
            }
        }
        repl_default = false;
    }

    let repl = repl || repl_default;

    if repl {
        jl.load(&mut INITREPL_JL.as_bytes(), Some("initrepl.jl"))
            .expect("Could not load initrepl.jl");
        interactive(jl, quiet);
    }
}
