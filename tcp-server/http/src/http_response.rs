use std::{
    collections::HashMap,
    io::{Error, Write},
};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(value: HttpResponse<'a>) -> Self {
        let res = &value;
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            res.version(),
            res.status_code(),
            res.status_text(),
            res.headers(),
            res.body().len(),
            res.body()
        )
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> Self {
        let mut response = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        };
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Not Found",
        };
        response.body = body;
        response
    }
    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<(), Error> {
        let res = self.clone();
        let response_string = String::from(res);
        let _ = write!(write_stream, "{}", response_string);
        Ok(())
    }

    pub fn version(&self) -> &str {
        self.version
    }
    pub fn status_code(&self) -> &str {
        self.status_code
    }
    pub fn status_text(&self) -> &str {
        self.status_text
    }
    pub fn headers(&self) -> String {
        let headers_map = self.headers.clone().unwrap();
        let mut headers = String::new();
        for (k, v) in headers_map.iter() {
            headers = format!("{}{}:{}\r\n", headers, k, v);
        }
        headers
    }
    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::HttpResponse;

    #[test]
    fn test_http_response_actual_200() {
        let http_response_200 = HttpResponse::new("200", None, Some("".to_string()));
        let http_response_excepted = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("".to_string()),
        };
        assert_eq!(http_response_200, http_response_excepted);
    }
    #[test]
    fn test_http_response_actual_404() {
        let http_response_200 = HttpResponse::new("404", None, Some("xxxx".to_string()));
        let http_response_excepted = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".to_string()),
        };
        assert_eq!(http_response_200, http_response_excepted);
    }
    #[test]
    fn test_http_response_into() {
        let http_response_excepted = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxx".to_string()),
        };
        let http_response_string: String = http_response_excepted.into();
        let actual_string =
            "HTTP/1.1 200 OK\r\nContent-Type:text/html\r\nContent-Length: 3\r\n\r\nxxx";
        assert_eq!(actual_string, http_response_string);
    }
}
