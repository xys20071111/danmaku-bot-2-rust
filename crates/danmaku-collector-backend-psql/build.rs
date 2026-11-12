use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(
        &[
            "protobuf/gift.proto",
            "protobuf/user_info.proto",
            "protobuf/interact_word.proto",
        ],
        &["protobuf/"],
    )?;
    Ok(())
}
