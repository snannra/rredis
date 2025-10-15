pub enum Response {
    Simple(String),
    Bulk(Option<String>),
    Error(String),
}

impl Response {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Response::Simple(s) => format!("{}\n", s).into_bytes(),
            Response::Bulk(Some(s)) => format!("{}\n", s).into_bytes(),
            Response::Bulk(None) => "(nil)\n".as_bytes().to_vec(),
            Response::Error(e) => format!("{}\n", e).into_bytes(),
        }
    }
}
