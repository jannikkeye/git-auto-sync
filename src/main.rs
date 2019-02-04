extern crate watchexec;
extern crate clap;

use git_auto_sync::{
    repo,
    WatchHandler,
    get_matches
};
use std::path;
use watchexec::{
    cli::Args,
    run::watch
};
use std::env;

fn main() {
    let matches = get_matches();

    let path_arg = matches.value_of("path").unwrap();
    let path = match path::Path::new(&path_arg).canonicalize() {
        Ok(path) => path,
        Err(err) => panic!(err),
    };

    env::set_current_dir(path.to_path_buf()).expect(&format!("Failed to change current directory to {:#?}", &path));

    if path.is_dir() == false {
        panic!("{:?} is not a directory", path);
    }

    if repo::is_repo(&path) == false {
        panic!(format!("{:#?} is not a git repository", path));
    }


    println!("Watching path: {:#?}", path);

    let script = "
    unstaged_files=`git diff --cached --numstat | wc -l`

    echo $unstaged_files unstaged files

    timestamp() {
        date +\"%T\"
    }

    git add .

    if [ $unstaged_files -eq 0 ]
    then
    echo no staged changes detected
    else
    git commit -a -m \"$(timestamp) – automatic sync\"
    git push
    fi
    ";

    let arglist = Args {
        filters: vec![],
        no_shell: false,
        once: true,
        signal: None,
        restart: true,
        poll: false,
        poll_interval: 50,
        debounce: 1500,
        ignores: vec![String::from("**/.git/**/*")],
        no_vcs_ignore: false,
        clear_screen: false,
        debug: false,
        run_initially: true,
        cmd: vec![
            script.to_owned(),
        ],
        paths: vec![path.to_path_buf()],
    };
    

    match watch::<WatchHandler>(arglist) {
        Ok(()) => println!("Watching path: {:#?}", &path),
        Err(e) => panic!(e),
    };
}
