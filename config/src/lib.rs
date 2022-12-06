pub struct Config {
    pub url_string_lim: usize,
    pub test_server_addr: &'static str,
    pub test_api_domain: &'static str,
    pub api_domain: &'static str,
}

impl Config {
    const fn default() -> Self {
        Self {
            url_string_lim: 100,
            test_server_addr: "127.0.0.1:8000",
            test_api_domain: "http://127.0.0.1:8000/api",
            api_domain: "/api",
        }
    }
}

pub const CONFIG: Config = Config::default();
