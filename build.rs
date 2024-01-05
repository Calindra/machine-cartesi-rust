fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("grpc-interfaces/versioning.proto")?;

    tonic_build::configure().build_server(false).compile(
        &[
            "grpc-interfaces/stateserver.proto",
            "grpc-interfaces/versioning.proto",
            "grpc-interfaces/cartesi-machine.proto",
        ],
        &["grpc-interfaces"],
    )?;

    Ok(())
}
