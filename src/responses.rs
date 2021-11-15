pub enum Response {
    OK,
    NotFound,
}

pub fn create_response(res_type: Response) -> String {
  match res_type {
      Response::OK => String::from("HTTP/1.1 200 OK\r\n\r\n"),
      Response::NotFound => String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n")
  }
}