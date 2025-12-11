use bollard::Docker;
use bollard::query_parameters::CreateImageOptions;
use bollard::query_parameters::StopContainerOptions;
use bollard::query_parameters::ListImagesOptions;
use bollard::query_parameters::ListContainersOptions;
use bollard::models::ContainerSummary;
use bollard::errors::Error;
use bollard::query_parameters::StartContainerOptions;
use bollard::secret::ImageSummary;
use futures_util::TryStreamExt;
// use bollard::secret::ContainerSummary;
pub struct DockerClient{
    docker: Docker,
}

impl DockerClient{
    pub fn new()-> Self{
        let docker = Docker::connect_with_named_pipe(
            "npipe:////./pipe/dockerDesktopLinuxEngine", 
            120, 
            bollard::API_DEFAULT_VERSION
        )
        .expect("failed to connect to docker.");
        Self {docker: docker}
    }

    pub async fn list_containers(&self, all: bool) -> Result<Vec<ContainerSummary>, Error> {
        let options = Some(ListContainersOptions{
            all,
            ..Default::default()
        });

        let contianers = self.docker.list_containers(options).await?;
        Ok(contianers)
    }

    pub async fn list_images(&self, all:bool) -> Result<Vec<ImageSummary>, Error> {
        let options = Some(ListImagesOptions{
            all,
            ..Default::default()
        });

        let imgases = self.docker.list_images(options).await?;
        Ok(imgases)
    }

    pub async fn start_container(&self, container_name: &str) -> Result<(), Error> {
        self.docker.start_container(container_name, None::<StartContainerOptions>).await
    }

    pub async fn stop_container(&self, container_name: &str) -> Result<(), Error> {
        self.docker.stop_container(container_name, None::<StopContainerOptions>).await
    }

    pub async fn pull_image(&self, image_name: &str) -> Result<(), Error> {
        // Implementation for pulling an image from a registry
        let options = Some(CreateImageOptions{
            from_image: Some(image_name.to_string()),
            ..Default::default()
        });
        let mut stream = self.docker.create_image(options, None, None);
        while let Some(msg) = stream.try_next().await? {
            if let Some(Status) = msg.status {
                println!("{}", Status);
            }
        }        
        Ok(())
    }
}