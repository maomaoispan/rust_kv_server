use super::CommandService;
use crate::{CommandResponse, Hget, Hgetall, Hset, KvError, Storage, Value};

impl CommandService for Hget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        let rst = store.get(&self.table, &self.key);

        match rst {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hgetall {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        let rst = store.get_all(&self.table);

        match rst {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hset {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match self.pair {
            Some(v) => {
                let rst = store.set(&self.table, v.key, v.value.unwrap_or_default());
                match rst {
                    Ok(Some(v)) => v.into(),
                    Ok(None) => Value::default().into(),
                    Err(e) => e.into(),
                }
            }
            None => Value::default().into(),
        }
    }
}
