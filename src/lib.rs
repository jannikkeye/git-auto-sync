use watchexec::{
    cli::Args,
    pathop::PathOp,
    error::Error,
    run::{Handler, ExecHandler}
};
use clap::{App, Arg};

pub mod repo;

pub struct WatchHandler {
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

        self.inner.on_update(ops)?;

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

pub fn get_matches<'a>() -> clap::ArgMatches<'a> {
    App::new("My Super Program")
        .version("1.0")
        .author("Jannik Keye <jannik.keye@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("path")
            .help("Path to watch")
            .index(1)
            .required(true)
        )
        .get_matches()
}