use cartesi_machine::{machine_client::MachineClient, Void};
use tonic::Request;

pub mod versioning {
    tonic::include_proto!("versioning");
}

pub mod server_state {
    tonic::include_proto!("state_server");
}

pub mod cartesi_machine {
    tonic::include_proto!("cartesi_machine");
}

// https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MachineClient::connect("http://[::1]:50051").await?;

    println!("Connected");

    // let request = Request::new(Void::default());
    // let response = client.get_default_config(request).await?;

    // println!("Configuration={:?}", response);

    let request = Request::new(Void::default());
    let response = client.get_version(request).await?;

    let version = response.into_inner().version;

    println!("Version={:?}", version);

    Ok(())
}
