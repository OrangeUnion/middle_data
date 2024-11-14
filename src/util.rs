use chrono::{Local, NaiveDateTime};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};
use void_log::log_info;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: Option<ConfigServer>,
    pub database: Option<ConfigDatabase>,
    pub redis: Option<ConfigRedis>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigServer {
    pub path: Option<String>,
    pub port: Option<i64>,
    pub tls: Option<bool>,
}

impl Default for ConfigServer {
    fn default() -> Self {
        Self { path: Some("0.0.0.0".to_string()), port: Some(9011), tls: Some(false) }
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ConfigDatabase {
    pub url: Option<String>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ConfigRedis {
    pub url: String,
    pub username: String,
    pub password: String,
    pub expire: i64,
}

impl Config {
    pub async fn new() -> Self {
        let mut yaml_file = File::open("config.yaml").await.expect("read file error");
        let mut yaml_str = String::new();
        yaml_file.read_to_string(&mut yaml_str).await.expect("read str error");
        serde_yml::from_str::<Config>(yaml_str.as_str()).expect("config error")
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
}

pub fn new_token(league_code: &str) -> String {
    let mut rng = rand::thread_rng();
    let random_key: Vec<u8> = (0..32).map(|_| rng.gen_range(0..=255)).collect();
    log_info!("{:?}", random_key);

    let claims = Claims {
        sub: format!("{league_code}@org.void"),
        company: league_code.to_string(),
    };
    let key = encode(&Header::default(), &claims, &EncodingKey::from_secret(random_key.as_ref())).unwrap_or(String::default());
    log_info!("NEW KEY {key}");
    key
}

pub fn now_time() -> NaiveDateTime {
    Local::now().naive_local()
}

/// # 兼容格式化时间
pub fn format_time(time: &str) -> NaiveDateTime {
    let fmt_vec = vec!["%Y-%m-%d %H:%M:%S", "%Y-%m-%dT%H:%M:%S", "%Y-%m-%d %H:%M", "%Y-%m-%dT%H:%M"];
    for fmt in fmt_vec {
        match NaiveDateTime::parse_from_str(&time, fmt) {
            Ok(res_time) => {
                return res_time;
            }
            Err(_) => { continue; }
        }
    }
    NaiveDateTime::default()
}
