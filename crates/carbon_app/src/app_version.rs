pub const APP_VERSION: &str = match option_env!("APP_VERSION") {
    Some(v) => v,
    None => "1.0.0",
};
