use client::machine_client::MachineClient;

pub mod client {
    tonic::include_proto!("cartesi_machine");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MachineClient::connect("http://[::1]:50051")?;

    // let request = tonic::Request::new(Empty {});

    // let response = client
    //     .get_status(request)
    //     .await?
    //     .into_inner();

    // println!("RESPONSE={:?}", response);

    Ok(())
}