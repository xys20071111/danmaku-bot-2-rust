use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["protobuf/gift.proto"], &["protobuf/"])?;
    Ok(())
}
