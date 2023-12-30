use std::collections::HashMap;
use std::io::{self, Write, Read};
use std::process::{Stdio};
use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;

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
            run: "see repl",
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
            // https://v2.ocaml.org/releases/5.1/api/index.html
            docs: "dmmulroy",
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
        map
    };
}

struct ResourcesV2 {
    build: Option<&'static str>,
    //run: Option<&'static str>,
    //docs: Option<&'static str>,
    //test: Option<&'static str>,
    //repl: Option<&'static str>,
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
}

impl Command {
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
            },
        };
        println!("{}", res);
    }
}

// return result from main
fn main() {
    // let args = Args::parse();
    /*
    println!("You want to {:?} with {:?}? Let me try to help you with that", args.command, args.lang);

    match M.get(&args.lang) {
        Some(r) => args.command.print_resource(r),
        None => todo!("Crap, I'm sorry I didn't do this yet for {:?}", args.lang),
    }
     */

    let mut child = std::process::Command::new("ls")
        .arg("-l")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let mut stdout = child.stdout.take().unwrap();

    let mut buffer = [0; 4096];

    let stdout_handle = io::stdout();
    let mut stdout_lock = stdout_handle.lock();
    loop {
        match stdout.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                stdout_lock.write_all(&buffer[0..n]).expect("write all failed");
                stdout_lock.flush().expect("flush failed");
            }
            Err(err) => {
                println!("{err:?}");
                return;
            }
        }
    }

    let status = child.wait().expect("wait failed");

    println!("{status}");

}
