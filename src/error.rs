use failure::Fail;
use reqwest::StatusCode;

#[derive(Fail, Debug)]
pub enum PocketcastError {
    #[fail(display = "empty response")]
    EmptyResponse,
    #[fail(display = "invalid http status code: {:?}", _0)]
    HttpStatusError(StatusCode),
    #[fail(display = "invalid credentials")]
    InvalidCredentials
}
