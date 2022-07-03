
pub fn exit(msg: &str) {
    error!("{}",msg);
    std::process::exit(1)
}

pub fn err (e: failure::Error) {
    error!("{:?}", e)
}
