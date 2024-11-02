use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = String::new();

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
            } else {
                parsed_msg_body = line.into();
            }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body,
        }
    }
}

fn process_req_line(line: &str) -> (Method, Resource, Version) {
    let mut words = line.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();
    (
        method.into(),
        Resource::Path(resource.into()),
        version.into(),
    )
}

fn process_header_line(line: &str) -> (String, String) {
    let mut header_items = line.split(":");
    let mut key = String::from("");
    let mut val = String::from("");

    if let Some(k) = header_items.next() {
        key = k.to_string();
    }

    if let Some(v) = header_items.next() {
        val = v.to_string();
    }

    (key, val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let v1: Version = "HTTP/1.1".into();
        assert_eq!(v1, Version::V1_1);
    }

    #[test]
    fn test_http_request_into() {
        let data = "GET /greeting HTTP/1.1\r\nHOST: localhost:3000\r\nAccept: */*\r\nUser-Agent: curl/7.71.1\r\n\r\nhello I'm message body".to_string();
        let mut headers_expected = HashMap::new();
        headers_expected.insert("HOST".to_string(), " localhost".to_string());
        headers_expected.insert("Accept".to_string(), " */*".to_string());
        headers_expected.insert("User-Agent".to_string(), " curl/7.71.1".to_string());

        let req: HttpRequest = data.into();
        assert_eq!(req.method, Method::Get);
        assert_eq!(req.resource, Resource::Path("/greeting".to_string()));
        assert_eq!(req.version, Version::V1_1);
        assert_eq!(req.headers, headers_expected);
        assert_eq!(req.msg_body, "hello I'm message body".to_string());
    }
}
