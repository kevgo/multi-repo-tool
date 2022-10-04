use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};
use std::env;
use std::path::PathBuf;
use std::process::Output;
use std::str;
// use tokio::fs;
// use tokio::fs::File;
// use tokio::io;
use tokio::process::Command;

#[derive(Debug, Default, World)]
pub struct MrtWorld {
    dir: Option<PathBuf>,
    output: Option<Output>,
}

#[given(expr = "I am in the {string} folder")]
async fn in_the_folder(world: &mut MrtWorld, folder: String) {
    let cwd = env::current_dir().expect("cannot determine current dir");
    world.dir = Some(cwd.join(cwd.join(folder)));
}

#[when(expr = "running {string}")]
async fn when_running(world: &mut MrtWorld, command: String) {
    let mut argv = command.split_ascii_whitespace();
    match argv.next() {
        Some("m") => {}
        _ => panic!("The end-to-end tests can only run the 'm' command"),
    }
    let cwd = env::current_dir().expect("cannot determine current dir");
    let mrt_path = cwd.join("target").join("debug").join("mrt");
    let examples_dir = world.dir.as_ref().unwrap();
    let output = Command::new(mrt_path)
        .args(argv)
        .current_dir(&examples_dir)
        .env("HOME", &examples_dir)
        .env("MRT_WRAPPED", "true")
        .output()
        .await
        .expect("cannot find the 'mrt' executable");
    world.output = Some(output);
}

#[then("it prints:")]
async fn it_prints(world: &mut MrtWorld, step: &Step) {
    let examples_dir = world.dir.as_ref().unwrap();
    let want = step.docstring().expect("step has no docstring");
    let want = want.replace("{{examples_path}}", &examples_dir.to_string_lossy());
    let output = world.output.take().expect("run has first");
    assert_eq!(str::from_utf8(&output.stdout).unwrap().trim(), want.trim());
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    MrtWorld::run("features").await;
}
