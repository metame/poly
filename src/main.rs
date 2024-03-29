#![allow(dead_code)]
use std::collections::HashMap;
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
            build: Some(Bin::new("make", Box::new([]))), // requires file(s)
            run: None,
            test: None,
            repl: None,
            docs: Some(Bin::new("open", Box::new(["https://en.cppreference.com/w/c"])))
        });
        map.insert(Lang::CPP, ResourcesV2 {
            build: Some(Bin::new("make", Box::new([]))), // requires file(s)
            run: None,
            test: None,
            repl: None,
            docs: Some(Bin::new("open", Box::new(["https://en.cppreference.com/w/cpp"])))
        });
        map.insert(Lang::Zig, ResourcesV2 {
            build: Some(Bin::new("zig", Box::new(["build"]))),
            run: Some(Bin::new("zig", Box::new(["run"]))),
            test: Some(Bin::new("zig", Box::new(["test"]))),
            repl: None,
            docs: Some(Bin::new("open", Box::new(["https://ziglang.org/documentation/master/std/#A;std"]))),
        });
        map.insert(Lang::Scheme, ResourcesV2 {
            build: None,
            run: None,
            test: None,
            repl: Some(Bin::new("guile", Box::new([]))),
            docs: Some(Bin::new("open", Box::new(["https://www.gnu.org/software/guile/manual/guile.html#API-Reference"])))
        });
        map.insert(Lang::CommonLisp, ResourcesV2 {
            build: None,
            run: None,
            test: None,
            repl: Some(Bin::new("sbcl", Box::new([]))),
            docs: Some(Bin::new("open", Box::new(["https://www.lispworks.com/documentation/lw51/CLHS/Front/X_AllSym.htm"])))
        });
        map.insert(Lang::Racket, ResourcesV2 {
            build: None,
            run: Some(Bin::new("racket", Box::new([]))),
            test: None,
            repl: Some(Bin::new("racket", Box::new([]))),
            docs: Some(Bin::new("open", Box::new(["https://docs.racket-lang.org/reference/index.html"])))
        });
        map.insert(Lang::Sml, ResourcesV2 {
            build: None,
            run: None,
            test: None,
            repl: Some(Bin::new("sml", Box::new([]))),
            docs: Some(Bin::new("open", Box::new(["https://smlfamily.github.io/Basis/overview.html"])))
        });
        map.insert(Lang::Haskell, ResourcesV2 {
            build: Some(Bin::new("stack", Box::new(["build"]))),
            run: Some(Bin::new("stack", Box::new(["run"]))),
            test: Some(Bin::new("stack", Box::new(["test"]))),
            repl: Some(Bin::new("stack", Box::new(["ghci"]))),
            docs: Some(Bin::new("open", Box::new(["https://hoogle.haskell.org/"])))
        });
        map.insert(Lang::Ocaml, ResourcesV2 {
            build: Some(Bin::new("dune", Box::new(["build"]))),
            run: Some(Bin::new("dune", Box::new(["exec"]))),
            test: Some(Bin::new("dune", Box::new(["test"]))),
            repl: Some(Bin::new("utop", Box::new([]))),
            docs: Some(Bin::new("open", Box::new(["https://twitch.tv/dmmulroy"])))
        });
        map.insert(Lang::Rust, ResourcesV2 {
            build: Some(Bin::new("cargo", Box::new(["build"]))),
            run: Some(Bin::new("cargo", Box::new(["run"]))),
            test: Some(Bin::new("cargo", Box::new(["test"]))),
            repl: Some(Bin::new("irust", Box::new([]))),
            // TODO: should we do `cargo doc --open` or url?
            docs: Some(Bin::new("open", Box::new(["https://doc.rust-lang.org/stable/std/"])))
        });
        map.insert(Lang::Clojure, ResourcesV2 {
            build: Some(Bin::new("lein", Box::new(["uberjar"]))),
            run: Some(Bin::new("lein", Box::new(["run"]))),
            test: Some(Bin::new("lein", Box::new(["test"]))),
            repl: Some(Bin::new("lein", Box::new(["repl"]))),
            docs: Some(Bin::new("open", Box::new(["https://clojuredocs.org"])))
        });
        map
    };
}

struct Bin {
    name: &'static str,
    args: Box<[&'static str]>,
}

impl Bin {
    fn new(name: &'static str, args: Box<[&'static str]>) -> Self {
        Bin { name, args }
    }

    fn run(&self, user_args: &Option<String>) {
        let args = if let Some(args) = user_args {
            let mut v = self.args.to_vec();
            v.push(&args);
            v
        } else {
            self.args.to_vec()
        };

        if self.name == "open" {
            // TODO: print failure of open to console
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
// TODO: required args?
// TODO: intelligent running of programs/commands based on context
// e.g. `poly ocaml run` would work without an arg in a dune project
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
                |b| b.run(&args.args),
            );
    }
}
