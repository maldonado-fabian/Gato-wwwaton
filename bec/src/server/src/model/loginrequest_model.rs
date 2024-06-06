use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub rut: String,
    pub pass: String,
}