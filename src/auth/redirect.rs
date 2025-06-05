/// HTTP server on host system
/// ex. 127.0.0.1:7878
/// blocks until one request is recieved (auth redirect) and parses it to get the code
pub struct Server {
    auth: super::OAuth,
    app_name: String,
}

/// Error type for server methods
#[derive(Debug)]
pub enum ServerError {
    IOError(std::io::Error),
    HTTParseError(httparse::Error),
    InvalidRequestURL(String),
    AuthError(super::AuthError),
}

impl From<std::io::Error> for ServerError {
    fn from(e: std::io::Error) -> Self {
        ServerError::IOError(e)
    }
}

impl From<httparse::Error> for ServerError {
    fn from(e: httparse::Error) -> Self {
        ServerError::HTTParseError(e)
    }
}

impl From<super::AuthError> for ServerError {
    fn from(e: super::AuthError) -> Self {
        ServerError::AuthError(e)
    }
}

impl Server {
    /// Create the server
    pub fn new<A: ToString>(app_name: A, auth: super::OAuth) -> Self {
        Server {
            auth,
            app_name: app_name.to_string(),
        }
    }

    /// Run the server.
    /// Blocks until it recieves exactly one request.
    pub fn go(self) -> Result<super::OAuth, ServerError> {
        use std::io::prelude::*;
        use std::net::TcpListener;

        let listener = TcpListener::bind(&self.auth.redirect_url)?;
        let mut socket_stream = listener.incoming().next().unwrap()?;

        // read all bytes of the request
        let mut request_bytes = Vec::new();
        loop {
            const BUF_SIZE: usize = 4096;
            let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
            match socket_stream.read(&mut buf) {
                Ok(val) => {
                    if val > 0 {
                        request_bytes.append(&mut Vec::from(&buf[0..val]));
                        if val < BUF_SIZE {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                Err(e) => panic!("{}", e),
            };
        }

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut parsed_request = httparse::Request::new(&mut headers);

        parsed_request.parse(&request_bytes)?;

        let raw_url = if let Some(path) = parsed_request.path {
            format!("http://{}{}", self.auth.redirect_url, path)
        } else {
            return Err(ServerError::InvalidRequestURL("".to_string()));
        };

        let parsed_url = match url::Url::parse(&raw_url) {
            Ok(url) => url,
            Err(_) => return Err(ServerError::InvalidRequestURL(raw_url)),
        };

        let query = if let Some(query) = parsed_url.query() {
            query
        } else {
            return Err(ServerError::InvalidRequestURL(
                "No query string".to_string(),
            ));
        };

        let mut ret_auth = self.auth;

        ret_auth.parse_redirect_query_string(query)?;

        // return a minimal http response to the browser
        let r = format!("HTTP/1.1 200 OK\r\n\r\n<html><head><title>{} Authorized</title></head><body>{} Authorized</body></html>", self.app_name, self.app_name);
        socket_stream.write_all(r.as_bytes())?;
        socket_stream.flush()?;

        Ok(ret_auth)
    }
}
