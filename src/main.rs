mod cli;
mod docker;
use clap::Parser;
use cli::{Cli, Command, ListCommands};
use docker::DockerClient;

#[tokio::main]
async fn main() {
    let args:Cli = Cli::parse();
    let docker_client = DockerClient::new();

    match args.command{
        Command::List { list_command } => match list_command {
            ListCommands::Containers { all }=>{
                match docker_client.list_containers(all).await {
                    Ok(containers) => {
                        for container in containers {
                            println!(
                                "{}\t{}\t{}",
                                container.id.unwrap_or_default(),
                                container.names.unwrap_or_default().join(" "),
                                container.status.unwrap_or_default()
                            )
                        }
                    }
                    Err(e)=> eprintln!("Error printintg the containers: {} ", e),
                }
            }
            ListCommands:: Images { all }=>{
                match docker_client.list_images(all).await {
                    Ok(images) => {
                        for image in images {
                            println!(
                                "{}\t{}",
                                image.id,
                                image.repo_tags.join(",")
                            )
                        }
                    }
                    Err(e)=> eprintln!("Error printintg the images: {} ", e),
                }
            }
        }
        Command::Start { container_name } => {
            match docker_client.start_container(&container_name).await {
                Ok(_) => println!("Container {} started successfully.", container_name),
                Err(e) => eprintln!("Error starting container {}: {}", container_name, e),
            }
        }
        Command::Stop { container_name } => {
            match docker_client.stop_container(&container_name).await {
                Ok(_) => println!("Container {} stopped successfully.", container_name),
                Err(e) => eprintln!("Error stopping container {}: {}", container_name, e),
           }
        }
        Command::Pull { image_name } => {
            match docker_client.pull_image(&image_name).await {
                Ok(_) => println!("Image {} pulled successfully.", image_name),
                Err(e) => eprintln!("Error pulling image {}: {}", image_name, e),
           }
        }
    }
    // println!("Hello, world!");
}
