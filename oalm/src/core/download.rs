extern crate reqwest; 
extern crate git2;

pub fn from_http(suposed_file: &str) -> Result<reqwest::Response, reqwest::Error> {
    let request = reqwest::get(suposed_file)?;
    
    Ok(request)
}