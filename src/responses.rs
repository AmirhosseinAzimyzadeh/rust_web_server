pub enum Response {
    OK,
    NotFound,
}

pub fn create_response(res_type: Response, contents: String) -> String {
  match res_type {
      Response::OK => format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents,
      ),
      Response::NotFound => String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n")
  }
}