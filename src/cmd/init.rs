use crate::cli::config::Config;
use crate::cli::docker::{Docker, DockerError};
use clap::{ArgMatches, Command};
use simplelog::*;
use spinners::{Spinner, Spinners};
use std::error::Error;
use std::process::Command as ShellCommand;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("init")
        .about("Initializes a local environment; generates configuration and pulls Docker image")
}

pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // check the system requirements
    match check_requirements() {
        Ok(_) => info!("Docker was found and appears running"),
        Err(e) => {
            return Err(e);
        }
    }

    // create the configuration file in the default location
    let config = Config::new(args, &Config::full_path(args));

    info!("Config file created at: {}", &config.created_at.to_string());

    // pull the Tembo image
    build_image()
}

fn check_requirements() -> Result<(), Box<dyn Error>> {
    Docker::installed_and_running()
}

fn build_image() -> Result<(), Box<dyn Error>> {
    if image_exist() {
        info!("Tembo image already exists, proceeding");
        return Ok(());
    }

    info!("Installing Tembo image");
    let mut sp = Spinner::new(Spinners::Line, String::new());
    let mut command = String::from("cd tembo"); // TODO: does this work for installed crates?
    command.push_str("&& docker build -t tembo-pg . ");
    command.push_str(" --quiet");

    let output = ShellCommand::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect("failed to execute process");

    sp.stop_with_newline();

    let stderr = String::from_utf8(output.stderr).unwrap();

    if !stderr.is_empty() {
        return Err(Box::new(DockerError::new(
            format!("There was an issue pulling the image: {}", stderr).as_str(),
        )));
    } else {
        info!("Tembo image was installed");
        Ok(())
    }
}

fn image_exist() -> bool {
    let command = String::from("docker images");
    let output = ShellCommand::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let image_name = String::from("tembo-pg-cnpg");
    let image = stdout.find(&image_name);

    image.is_some()
}
