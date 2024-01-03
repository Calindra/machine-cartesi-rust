fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("grpc-interfaces/cartesi-machine-checkin.proto")?;
    tonic_build::compile_protos("grpc-interfaces/cartesi-machine.proto")?;

    Ok(())
}
