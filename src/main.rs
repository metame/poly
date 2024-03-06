#![allow(dead_code)]
use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::io::Error;
use std::process::Stdio;

use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;
use open;

struct Resources {
    build: &'static str,
    run: &'static str,
    docs: &'static str,
    test: &'static str,
    repl: Option<&'static str>,
}

lazy_static! {
    static ref M: HashMap<Lang, Resources> = {
        let mut map = HashMap::new();
        map.insert(Lang::C, Resources {
            build: "clang",
            run: "uhh, run the binary",
            docs: "https://en.cppreference.com/w/c",
            test: "good luck",
            repl: None,
        });
        map.insert(Lang::CPP, Resources {
            build: "clang++",
            run: "uhh, run the binary",
            docs: "https://en.cppreference.com/w/cpp",
            test: "good luck",
            repl: None,
        });
        map.insert(Lang::Zig, Resources {
            build: "zig build",
            run: "zig run",
            docs: "https://ziglang.org/documentation/master/std/#A;std",
            test: "zig test",
            repl: None,
        });
        map.insert(Lang::Scheme, Resources {
            build: "see repl",
            run: "see repl",
            docs: "https://www.gnu.org/software/guile/manual/guile.html#API-Reference",
            test: "see repl",
            repl: Some("guile"),
        });
        map.insert(Lang::CommonLisp, Resources {
            build: "see repl",
            run: "see repl",
            docs: "https://www.lispworks.com/documentation/lw51/CLHS/Front/X_AllSym.htm",
            test: "see repl",
            repl: Some("sbcl"),
        });
        map.insert(Lang::Racket, Resources {
            build: "see repl",
            run: "racket <file_name>",
            docs: "https://docs.racket-lang.org/reference/index.html",
            test: "see repl",
            repl: Some("racket"),
        });
        map.insert(Lang::Sml, Resources {
            build: "- CM.make()",
            run: "load files into repl using `use \"myfile.sml\"`",
            docs: "https://smlfamily.github.io/Basis/overview.html",
            test: "good luck",
            repl: Some("sml"),
        });
        map.insert(Lang::Haskell, Resources {
            build: "stack build",
            run: "stack run",
            docs: "https://hoogle.haskell.org/",
            test: "stack test",
            repl: Some("stack ghci"),
        });
        map.insert(Lang::Ocaml, Resources {
            build: "dune build",
            run: "dune exec <project_name>",
            docs: "https://v2.ocaml.org/releases/5.1/api/index.html",
            test: "dune test",
            repl: Some("utop"),
        });
        map.insert(Lang::Rust, Resources {
            build: "cargo build",
            run: "cargo run",
            docs: "https://doc.rust-lang.org/stable/std/",
            test: "cargo test",
            repl: Some("cargo install irust && irust"),
        });
        map.insert(Lang::Clojure, Resources {
            build: "lein uberjar",
            run: "lein run",
            docs: "https://clojuredocs.org",
            test: "lein test",
            repl: Some("lein repl"),
        });
        map
    };

    static ref MV2: HashMap<Lang, ResourcesV2> = {
        let mut map = HashMap::new();
        map.insert(Lang::C, ResourcesV2 {
            build: Some(Bin::none("make")), // requires file(s)
            run: None,
            test: None,
            repl: None,
            docs: Some(Bin::one("open", "https://en.cppreference.com/w/c"))
        });
        map.insert(Lang::CPP, ResourcesV2 {
            build: Some(Bin::none("make")), // requires file(s)
            run: None,
            test: None,
            repl: None,
            docs: Some(Bin::one("open", "https://en.cppreference.com/w/cpp"))
        });
        map.insert(Lang::Zig, ResourcesV2 {
            build: Some(Bin::one("zig", "build")),
            run: Some(Bin::one("zig", "run")),
            test: Some(Bin::one("zig", "test")),
            repl: None,
            docs: Some(Bin::one("open", "https://ziglang.org/documentation/master/std/#A;std")),
        });
        map.insert(Lang::Scheme, ResourcesV2 {
            build: None,
            run: None,
            test: None,
            repl: Some(Bin::none("guile")),
            docs: Some(Bin::one("open", "https://www.gnu.org/software/guile/manual/guile.html#API-Reference"))
        });
        map.insert(Lang::CommonLisp, ResourcesV2 {
            build: None,
            run: None,
            test: None,
            repl: Some(Bin::none("sbcl")),
            docs: Some(Bin::one("open", "https://www.lispworks.com/documentation/lw51/CLHS/Front/X_AllSym.htm"))
        });
        map.insert(Lang::Racket, ResourcesV2 {
            build: None,
            run: Some(Bin::new("racket", Box::new([]), 1)),
            test: None,
            repl: Some(Bin::none("racket")),
            docs: Some(Bin::one("open", "https://docs.racket-lang.org/reference/index.html"))
        });
        map.insert(Lang::Sml, ResourcesV2 {
            build: None,
            run: None,
            test: None,
            repl: Some(Bin::none("sml")),
            docs: Some(Bin::one("open", "https://smlfamily.github.io/Basis/overview.html"))
        });
        map.insert(Lang::Haskell, ResourcesV2 {
            build: Some(Bin::one("stack", "build")),
            run: Some(Bin::one("stack", "run")),
            test: Some(Bin::one("stack", "test")),
            repl: Some(Bin::one("stack", "ghci")),
            docs: Some(Bin::one("open", "https://hoogle.haskell.org/"))
        });
        map.insert(Lang::Ocaml, ResourcesV2 {
            build: Some(Bin::one("dune", "build")),
            run: Some(Bin::new("dune", Box::new(["exec"]), 2)),
            test: Some(Bin::one("dune", "test")),
            repl: Some(Bin::none("utop")),
            docs: Some(Bin::one("open", "https://twitch.tv/dmmulroy"))
        });
        map.insert(Lang::Rust, ResourcesV2 {
            build: Some(Bin::one("cargo", "build")),
            run: Some(Bin::one("cargo", "run")),
            test: Some(Bin::one("cargo", "test")),
            repl: Some(Bin::none("irust")),
            // TODO: should we do `cargo doc --open` or url?
            docs: Some(Bin::one("open", "https://doc.rust-lang.org/stable/std/"))
        });
        map.insert(Lang::Clojure, ResourcesV2 {
            build: Some(Bin::one("lein", "uberjar")),
            run: Some(Bin::one("lein", "run")),
            test: Some(Bin::one("lein", "test")),
            repl: Some(Bin::one("lein", "repl")),
            docs: Some(Bin::one("open", "https://clojuredocs.org"))
        });
        map
    };
}

struct Bin {
    name: &'static str,
    args: Box<[&'static str]>,
    required_args: usize,
}

fn find_racket_source(e: Result<DirEntry, Error>) -> Option<String> {
    if let Ok(entry) = e {
        let p = entry.path();
        match p.extension() {
            Some(ext) if ext.to_str() == Some("rkt") => {
                p.to_str().map(|s| s.to_string())
            }
            _ => None,
        }
    } else {
        None
    }
}

fn find_ocaml_project(e: Result<DirEntry, Error>) -> Option<String> {
    if let Ok(entry) = e {
        let p = entry.path();
        match (p.extension(), p.file_stem()) {
            (Some(ext), Some(stem)) if ext.to_str() == Some("opam") => {
                stem.to_str().map(|s| s.to_string())
            }
            _ => None,
        }
    } else {
        None
    }
}

impl Bin {
    fn new(name: &'static str, args: Box<[&'static str]>, required_args: usize) -> Self {
        Bin { name, args, required_args }
    }

    fn none(name: &'static str) -> Self {
        Self::new(name, Box::new([]), 0)
    }

    fn one(name: &'static str, arg: &'static str) -> Self {
        Self::new(name, Box::new([arg]), 1)
    }

    fn run(&self, user_args: &Option<String>) -> Result<(), std::io::Error> {
        // TODO: use a Cow here instead
        let mut p = String::new();

        let args = if let Some(args) = user_args {
            let mut v = self.args.to_vec();
            v.push(&args);
            v
        } else if self.args.len() != self.required_args {
            if self.name == "dune" && self.args[0] == "exec" {
                let mut entries = fs::read_dir("./")?;
                p = entries
                    .find_map(find_ocaml_project)
                    .expect("No project found in directory, try specifying with --args");
                let mut v = self.args.to_vec();
                println!("Found project {}", &p);
                v.push(p.as_str());
                v
            } else if self.name == "racket" {
                let mut entries = fs::read_dir("./")?;
                p = entries
                    .find_map(find_racket_source)
                    .or_else(|| {
                        fs::read_dir("./src").ok()
                            .map(|mut es| es.find_map(find_racket_source))
                            .flatten()
                    })
                    .expect("No racket file found to run, try specifying with --args");
                let mut v = self.args.to_vec();
                println!("Found source {}", &p);
                v.push(p.as_str());
                v
            } else {
                self.args.to_vec()
            }
        } else {
            self.args.to_vec()
        };

        if self.name == "open" {
            open::that(args[0]).expect("failed to open");
        } else {
            std::process::Command::new(self.name)
                .args(args)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("failed to execute process")
                .wait()
                .expect("wait failed");
        }

        let _ = p;

        Ok(())
    }
}

struct ResourcesV2 {
    build: Option<Bin>,
    run: Option<Bin>,
    test: Option<Bin>,
    repl: Option<Bin>,
    docs: Option<Bin>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, ValueEnum)]
enum Lang {
    C,
    CPP,
    Zig,
    Scheme,
    CommonLisp,
    Racket,
    Sml,
    Haskell,
    Ocaml,
    Coq,
    Agda,
    Idris,
    Rust,
    Clojure,
}

#[derive(Clone, Debug, ValueEnum)]
enum Command {
    Build,
    Run,
    Docs,
    Test,
    Repl,
    // TODO: init project?
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    lang: Lang,
    command: Command,
    #[arg(short, long)]
    dry: bool,
    #[arg(short, long)]
    args: Option<String>,
}

impl Command {
    fn get_resource<'a>(&self, r: &'a ResourcesV2) -> Option<&'a Bin> {
        match self {
            Self::Build => r.build.as_ref(),
            Self::Run => r.run.as_ref(),
            Self::Test => r.test.as_ref(),
            Self::Repl => r.repl.as_ref(),
            Self::Docs => r.docs.as_ref(),
        }
    }

    fn print_resource(&self, r: &Resources) {
        let res = match self {
            Self::Build => r.build,
            Self::Run => r.run,
            Self::Docs => r.docs,
            Self::Test => r.test,
            Self::Repl => {
                if let Some(repl) = r.repl {
                    repl
                } else {
                    "it don't have none"
                }
            }
        };
        println!("{}", res);
    }
}

fn dry_run(lang: &Lang, cmd: &Command, eff: bool) {
    if eff {
        println!(
            "I don't know how to do this yet but here's how you can do it yourself (hopefully)"
        );
    }
    match M.get(lang) {
        Some(r) => cmd.print_resource(r),
        None => todo!(
            "Crap, I'm sorry I don't know how to {:?} yet for {:?}",
            cmd,
            lang
        ),
    }
}

// return result from main
// TODO?: change --args to trailing_var_arg (won't do until I have a reason to)
// https://docs.rs/clap/latest/clap/struct.Arg.html#method.trailing_var_arg
// TODO: add rust-analyzer to nix flake, update hash in nix flake
fn main() {
    let args = Args::parse();

    let s = if args.dry { "know how to " } else { "" };
    println!(
        "You want to {s}{:?} with {:?}? Let me try to help you with that",
        args.command, args.lang
    );

    if args.dry {
        dry_run(&args.lang, &args.command, false);
    } else {
        MV2.get(&args.lang)
            .and_then(|r| args.command.get_resource(r))
            .map_or_else(
                || dry_run(&args.lang, &args.command, true),
                |b| b.run(&args.args).expect("run failed you bastards"),
            );
    }
}
