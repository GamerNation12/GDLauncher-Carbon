macro_rules! get_base_api_env {
    () => {{
        let version_json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/config/version.json"
        ));

        let version: serde_json::Value = serde_json::from_str(version_json).unwrap();
        if version["channel"] == "snapshot" || cfg!(debug_assertions) {
            option_env!("TEST_BASE_API").unwrap_or("https://api.gdlauncher.com").to_string()
        } else {
            option_env!("BASE_API").unwrap_or("https://api.gdlauncher.com").to_string()
        }
    }};
}

pub(crate) use get_base_api_env;
