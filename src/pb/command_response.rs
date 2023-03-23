use crate::{CommandResponse, KvError, Kvpair, Value};
use http::StatusCode;

impl From<Value> for CommandResponse {
    fn from(v: Value) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![v],
            ..Default::default()
        }
    }
}

impl From<Vec<Value>> for CommandResponse {
    fn from(values: Vec<Value>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![],
            ..Default::default()
        }
    }
}

impl From<Vec<Kvpair>> for CommandResponse {
    fn from(pairs: Vec<Kvpair>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            pairs: pairs,
            ..Default::default()
        }
    }
}

impl From<KvError> for CommandResponse {
    fn from(e: KvError) -> Self {
        let status: u32 = match e {
            KvError::NotFound(_, _) => StatusCode::NOT_FOUND.as_u16() as _,
            KvError::InvalidCommand(_) => StatusCode::BAD_REQUEST.as_u16() as _,
            _ => StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _,
        };

        Self {
            status,
            message: e.to_string(),
            values: vec![],
            pairs: vec![],
        }
    }
}
