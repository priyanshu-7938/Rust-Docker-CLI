# Docker Client CLI - (Compatable with windows-Doker-Daemon)

A command-line interface (CLI) tool written in Rust for interacting with the Docker daemon. This tool provides a simple way to manage Docker containers and images directly from your terminal, without needing to type full `docker` commands.

## Features

- **List Containers**: Display running or all Docker containers with their ID, names, and status.
- **List Images**: Show Docker images with their ID and repository tags.
- **Start Container**: Start a specified Docker container.
- **Stop Container**: Stop a specified Docker container.
- **Pull Image**: Download a Docker image from a registry like Docker Hub.

## Installation

To build and run this project, you need to have [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed on your system.

1.  **Clone the repository**:
    ```sh
    git clone https://github.com/priyanshu-7938/Rust-Docker-CLI.git
    cd Rust-Docker-CLI
    ```

2.  **Build the project**:
    This command will compile the project in release mode, creating an optimized executable.
    ```sh
    cargo build --release
    ```

3.  **Run the executable**:
    The compiled executable will be located in the `target/release/` directory. On Windows, it will be named `docker_client_cli.exe`.
    You can run it directly:
    ```sh
    target\release\docker_client_cli.exe --help
    ```
    For easier use, consider adding the `target\release\` directory to your system's PATH environment variable or creating an alias for `docker_client_cli.exe`.

## Usage

The CLI follows a simple command structure. Here are the available commands and their usage:

### List Containers

Lists Docker containers. By default, it lists only running containers. Use the `--all` flag to list all containers (including stopped ones).

```sh
# List only running containers
docker_client_cli list containers

# List all containers (including stopped ones)
docker_client_cli list containers --all

```

### List Images

```sh
# List only top-level images
docker_client_cli list images

# List all images (including intermediate and dangling)
docker_client_cli list images --all
```

### Start/Stop Containers

```sh
docker_client_cli start <container_name_or_id>

docker_client_cli stop <container_name_or_id>
```
### Pull images

```sh
docker_client_cli pull <image_name>
```

### Dependencies
This project relies on the following Rust crates:

    - bollard: A Docker API client library for Rust, used for interacting with the Docker daemon.
    - clap: A powerful, easy-to-use, and full-featured command-line argument parser.
    - tokio: An asynchronous runtime for Rust, enabling non-blocking I/O.
    - futures: A set of tools for working with asynchronous programming in Rust.