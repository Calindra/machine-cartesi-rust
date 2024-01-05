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
    let addr = "http://[::1]:50051";
    let mut client = MachineClient::connect(addr).await?;

    println!("Connecting to remote cartesi machine at {}", addr);

    // let request = Request::new(Void::default());
    // let response = client.get_default_config(request).await?;

    // println!("Configuration={:?}", response);

    let request = Request::new(Void::default());
    let response = client.get_version(request).await?;

    let version = response
        .into_inner()
        .version
        .map(|v| format!("{}.{}.{}", v.major, v.minor, v.patch))
        .unwrap_or("unknown".to_string());

    println!("Connected: remote version is {}", version);

    // read iflags_H and iflags_Y

    Ok(())
}
