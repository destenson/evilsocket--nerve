#[macro_use]
extern crate anyhow;

use crate::agent::task::Task;
use agent::{generator, task::tasklet::Tasklet, Agent};
use clap::Parser;
use colored::Colorize;

mod agent;
mod cli;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    let generator = generator::factory(
        "ollama",
        &args.generator_url,
        args.generator_port,
        &args.model_name,
    )
    .expect("could not create generator");

    println!(
        "using {}@{}:{}",
        args.model_name.bold(),
        args.generator_url.dimmed(),
        args.generator_port.to_string().dimmed()
    );

    let mut tasklet: Tasklet =
        Tasklet::from_yaml_file(&args.tasklet).expect("could not read tasklet yaml file");
    if tasklet.prompt.is_none() {
        tasklet.prompt = Some(if let Some(prompt) = &args.prompt {
            prompt.to_string()
        } else {
            cli::get_user_input("enter task> ")
        });
    }
    let task = Box::new(tasklet);

    println!(
        "{}: {}",
        "task".bold(),
        task.to_prompt()
            .expect("could not convert task to prompt")
            .trim()
            .yellow()
    );

    let mut agent =
        Agent::new(generator, task, args.to_agent_options()).expect("could not create agent");

    println!(
        "{}: {}\n",
        "namespaces".bold(),
        agent.state().used_namespaces().join(", ")
    );

    while !agent.is_state_complete() {
        if let Err(error) = agent.step().await {
            println!("{}", error.to_string().bold().red());
            break;
        }
    }
}