use crate::{
    command_request::RequestData, CommandRequest, CommandResponse, KvError, Kvpair, Value,
};

///
/// 对 Command 的处理的抽象
///
pub trait CommandService {
    /// 处理 Command, 返回 Response
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

/// 对存储的抽象，我们不关心数据存在哪儿，但需要定义外界如何和存储打交道
pub trait Storage {
    ///从一个 HashTable 里获取一个 key 的 value
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;

    ///从一个 HashTable 里设置一个 key 的value， 返回就的 value
    fn set(&self, table: &str, key: &str, value: Value) -> Result<Option<Value>, KvError>;

    ///查看 HashTable 中是否有 key
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;

    /// 从 HashTable 中删除一个key
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;

    /// 遍历 HashTable， 返回所有 kv pair （这个接口不好)
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
}

// 从 Request 中得到 Response， 目前处理 HGET/HGETALL/HSET
pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Hget(param)) => param.execute(store),
        None => todo!(),
        _ => todo!(),
    }
}
