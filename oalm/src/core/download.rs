extern crate git2;
extern crate reqwest;

pub fn from_http(suposed_file: &str) -> Result<reqwest::Response, reqwest::Error> {
    let request = reqwest::get(suposed_file)?;

    Ok(request)
}
