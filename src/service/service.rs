use std::sync::Arc;

use tracing::debug;

use crate::{
    command_request::RequestData, pb::CommandService, CommandRequest, CommandResponse, KvError,
    MemTable, Storage,
};

struct ServiceInner<T> {
    store: T,
}

pub struct Service<T = MemTable> {
    inner: Arc<ServiceInner<T>>,
}

impl<T> Clone for Service<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T: Storage> Service<T> {
    pub fn new(store: T) -> Self {
        Self {
            inner: Arc::new(ServiceInner { store }),
        }
    }

    pub fn run(&self, cmd: CommandRequest) -> CommandResponse {
        debug!("Got request: {:?}", cmd);
        let res = dispatch(cmd, &self.inner.store);
        debug!("Executed response: {:?}", res);

        res
    }
}

pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Hset(v)) => v.execute(store),
        Some(RequestData::Hget(v)) => v.execute(store),
        Some(RequestData::Hdel(v)) => v.execute(store),

        Some(RequestData::Hgetall(v)) => v.execute(store),

        None => KvError::InvalidCommand("Request has no data".into()).into(),
        _ => KvError::Internal("Not implemented".into()).into(),
    }
}
