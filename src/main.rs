use cartesi_machine::machine_client::MachineClient;
use server_state::GetStateRequest;
use tonic::{transport::Channel, Request};

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
    let machine = MachineClient::connect("http://[::1]:50051").await?;

    println!("Connected");

    let response = Request::new(GetStateRequest {
        json_initial_state: String::from("{}"),
    });

    println!("Requesting state {:?}", response);


    // let response = client
    //     .get_status(request)
    //     .await?
    //     .into_inner();

    // println!("RESPONSE={:?}", response);

    Ok(())
}
