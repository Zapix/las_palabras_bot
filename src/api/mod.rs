mod info;
pub use info::info;

pub async fn health() -> String {
    "OK".to_string()
}
