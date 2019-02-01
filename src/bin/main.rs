extern crate watchexec;
extern crate clap;

use std::path;
use clap::{Arg, App};
use watchexec::{
    cli::Args,
    pathop::PathOp,
    error::Error,
    run::{watch, Handler, ExecHandler}
};


struct WatchHandler {
    inner: ExecHandler,
}

impl Handler for WatchHandler {
    fn new(args: Args) -> Result<Self, Error> {
        Ok(WatchHandler {
            inner: ExecHandler::new(args.clone())?,
        })
    }

    fn on_update(&mut self, ops: &[PathOp]) -> Result<bool, Error> {
        self.start();

        self.inner.on_update(ops);

        Ok(true)
    }

    fn on_manual(&mut self) -> Result<bool, Error> {
        self.start();

        Ok(true)
    }
}

impl WatchHandler {
    fn start(&self) {
        println!("Handling changes...");
    }
}

fn main() {
    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("path")
            .help("Path to watch")
            .index(1)
            .default_value(".")
            .required(true)
        )
        .arg(Arg::with_name("restart")
            .help("Restart the process if it's still running")
            .short("r")
            .long("restart"))
        .arg(Arg::with_name("debounce")
            .help("Set the timeout between detected change and command execution, defaults to 500ms")
            .takes_value(true)
            .value_name("milliseconds")
            .short("d")
            .long("debounce"))
        .get_matches();

    let path_arg = matches.value_of("path").unwrap();
    let path = path::Path::new(&path_arg).canonicalize().unwrap();

    if path.is_dir() == false {
        panic!("{:?} is not a directory", path);
    }

    println!("Watching path: {:#?}", path);

    let arglist = Args {
        filters: vec![],
        no_shell: false,
        once: true,
        signal: None,
        restart: true,

        poll: false,
        poll_interval: 50,
        debounce: 500,
        ignores: vec![String::from(".git")],
        no_vcs_ignore: false,

        clear_screen: false,
        debug: true,
        run_initially: true,

        cmd: vec![
            String::from("git add -A &&"),
            String::from("git commit -m \"automatic sync\" &&"),
            String::from("git push"),
        ],
        paths: vec![path.to_path_buf()],
    };
    

    match watch::<WatchHandler>(arglist) {
        Ok(()) => println!("Watching path: {:#?}", &path),
        Err(e) => panic!(e),
    };
}
