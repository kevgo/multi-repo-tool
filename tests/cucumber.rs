use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};
use std::env;
use std::path::PathBuf;
use std::process::Output;
use std::str;
use tokio::fs;
// use tokio::fs::File;
// use tokio::io;
use tokio::process::Command;

#[derive(Debug, Default, World)]
pub struct MrtWorld {
    dir: Option<PathBuf>,
    output: Option<Output>,
    previous_state: Option<String>,
}

#[given(expr = "I am in the {string} example folder")]
async fn in_the_folder(world: &mut MrtWorld, folder: String) {
    let cwd = env::current_dir().expect("cannot determine current dir");
    world.dir = Some(cwd.join("examples").join(folder));
}

#[given(expr = "I am in the middle of running {string}")]
async fn previously_ran(world: &mut MrtWorld, command: String) {
    let mut argv = command.split_ascii_whitespace();
    match argv.next() {
        Some("m") => {}
        _ => panic!("The end-to-end tests can only run the 'm' command"),
    }
    let cwd = env::current_dir().expect("cannot determine current dir");
    let mrt_path = cwd.join("target").join("debug").join("mrt");
    let examples_dir = world.dir.as_ref().unwrap();
    let home_dir = cwd.join("examples").join("home");
    let output = Command::new(mrt_path)
        .args(argv)
        .current_dir(&examples_dir)
        .env("HOME", &home_dir)
        .env("MRT_WRAPPED", "true")
        .output()
        .await
        .expect("cannot find the 'mrt' executable");
    world.output = Some(output);
    let config_path = home_dir.join(".config").join("mrt.json");
    let state = fs::read_to_string(config_path).await.unwrap();
    world.previous_state = Some(state);
}

#[given("no mrt configuration")]
async fn no_config(_world: &mut MrtWorld) {
    let cwd = env::current_dir().expect("cannot determine current dir");
    let home_dir = cwd.join("examples").join("home");
    let config_path = home_dir.join(".config").join("mrt.json");
    let _ = fs::remove_file(config_path).await;
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
    let home_dir = cwd.join("examples").join("home");
    let output = Command::new(&mrt_path)
        .args(argv)
        .current_dir(&examples_dir)
        .env("HOME", &home_dir)
        .env("MRT_WRAPPED", "true")
        .output()
        .await
        .expect("cannot find the 'mrt' executable");
    world.output = Some(output);
}

#[when(expr = "trying to run {string}")]
async fn trying_to_run(world: &mut MrtWorld, command: String) {
    let mut argv = command.split_ascii_whitespace();
    match argv.next() {
        Some("m") => {}
        _ => panic!("The end-to-end tests can only run the 'm' command"),
    }
    let cwd = env::current_dir().expect("cannot determine current dir");
    let mrt_path = cwd.join("target").join("debug").join("mrt");
    let examples_dir = world.dir.as_ref().unwrap();
    let home_dir = cwd.join("examples").join("home");
    let output = Command::new(&mrt_path)
        .args(argv)
        .current_dir(&examples_dir)
        .env("HOME", &home_dir)
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
    let want = want.replace("{{examples_dir}}", &examples_dir.to_string_lossy());
    let output = world.output.take().expect("no execution recorded");
    let printed = format!(
        "{}{}",
        str::from_utf8(&output.stdout).unwrap(),
        str::from_utf8(&output.stderr).unwrap()
    );
    assert_eq!(printed.trim(), want.trim());
}

#[then(expr = "the exit code is {string}")]
async fn verify_exit_code(world: &mut MrtWorld, want: String) {
    let success = world
        .output
        .as_ref()
        .expect("no run recorded")
        .status
        .success();
    match want.as_ref() {
        "success" => assert!(success),
        "failure" => assert!(!success),
        other => panic!("unknown exit code: {}", other),
    }
}

#[then("the saved state is now:")]
async fn verify_saved_state(world: &mut MrtWorld, step: &Step) {
    let cwd = env::current_dir().expect("cannot determine current dir");
    let home_dir = cwd.join("examples").join("home");
    let config_path = home_dir.join(".config").join("mrt.json");
    let have = fs::read_to_string(config_path).await.unwrap();
    let examples_dir = world.dir.as_ref().unwrap();
    let want = step.docstring().expect("step has no docstring");
    let want = want.replace("{{examples_dir}}", &examples_dir.to_string_lossy());
    assert_eq!(have.trim(), want.trim());
}

#[then("the saved state is unchanged")]
async fn verify_saved_state_unchanged(world: &mut MrtWorld) {
    let cwd = env::current_dir().expect("cannot determine current dir");
    let home_dir = cwd.join("examples").join("home");
    let config_path = home_dir.join(".config").join("mrt.json");
    let have = fs::read_to_string(config_path).await.unwrap();
    let want = world.previous_state.take().expect("no previous state");
    assert_eq!(have.trim(), want.trim());
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    MrtWorld::cucumber()
        .max_concurrent_scenarios(1) // only one concurrent scenario because examples/home/.config contains shared mutable state
        .run_and_exit("features")
        .await;
}
