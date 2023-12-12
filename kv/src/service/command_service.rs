use crate::{
    storage::{MemTable, Storage},
    CommandResponse, Hdel, Hexist, Hget, Hgetall, Hset, KvError, Value,
};

///
/// 对 Command 的处理的抽象
///
pub trait CommandService {
    /// 处理 Command, 返回 Response
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

pub trait Service<Store = MemTable> {}

// --- impl command service start

impl CommandService for Hgetall {
    fn execute(self, store: &impl crate::command_service::Storage) -> CommandResponse {
        match store.get_all(&self.table) {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hset {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match self.pair {
            Some(v) => match store.set(&self.table, v.key, v.value.unwrap_or_default()) {
                Ok(Some(v)) => v.into(),
                Ok(None) => Value::default().into(),
                Err(e) => e.into(),
            },
            None => Value::default().into(),
        }
    }
}

impl CommandService for Hexist {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.contains(&self.table, &self.key) {
            Ok(v) => Value::from(v).into(),
            Err(_e) => KvError::NotFound(self.table, self.key).into(),
        }
    }
}

impl CommandService for Hdel {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.del(&self.table, &self.key) {
            Ok(Some(value)) => value.into(),
            Ok(None) => Value::default().into(),
            Err(e) => e.into(),
        }
    }
}
// --- impl command service end

#[cfg(test)]
mod tests {
    use crate::{
        assert_res_error, assert_res_ok,
        command_request::RequestData,
        storage::{MemTable, Storage},
        CommandRequest, CommandResponse, CommandService, Kvpair, Value,
    };

    #[test]
    fn command_should_work_for_memtable() {
        let store = MemTable::new();
        hset_should_work(&store);

        hget_should_work(&store);

        hget_with_non_exist_key_should_return_404(&store);

        hgetall_should_work(&store);
    }

    fn hgetall_should_work(store: &impl Storage) {
        let cmds = vec![
            CommandRequest::new_hset("score", "u1", 10.into()),
            CommandRequest::new_hset("score", "u2", 8.into()),
            CommandRequest::new_hset("score", "u3", 11.into()),
            CommandRequest::new_hset("score", "u1", 6.into()),
        ];

        for cmd in cmds {
            dispatch(cmd, store);
        }

        let cmd = CommandRequest::new_hgetall("score");
        let res = dispatch(cmd, store);

        let paris = &[
            Kvpair::new("u1", 6.into()),
            Kvpair::new("u2", 8.into()),
            Kvpair::new("u3", 11.into()),
        ];
        assert_res_ok(res, &[], paris);
    }

    fn hget_should_work(store: &impl Storage) {
        let cmd = CommandRequest::new_hset("source", "u1", 10.into());
        let _res = dispatch(cmd, store);
        let cmd = CommandRequest::new_hget("source", "u1");
        let res = dispatch(cmd, store);
        assert_res_ok(res, &[10.into()], &[]);
    }

    fn hget_with_non_exist_key_should_return_404(store: &impl Storage) {
        let cmd = CommandRequest::new_hget("t1", "non_exist_key");
        let res = dispatch(cmd, store);
        assert_res_error(res, 404, "Not found");
    }

    fn hset_should_work(store: &impl Storage) {
        let cmd = CommandRequest::new_hset("t1", "hello", "world".into());
        let res = dispatch(cmd.clone(), store);
        assert_res_ok(res, &[Value::default()], &[]);

        let res = dispatch(cmd, store);
        assert_res_ok(res, &["world".into()], &[]);
    }

    // 从 Request 中得到 Response， 目前处理 HGET/HGETALL/HSET
    fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
        match cmd.request_data {
            Some(RequestData::Hget(v)) => v.execute(store),
            Some(RequestData::Hgetall(v)) => v.execute(store),
            Some(RequestData::Hset(v)) => v.execute(store),
            _ => todo!(),
        }
    }
}
