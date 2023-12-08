pub use self::command_service::CommandService;
use crate::{command_request::RequestData, storage::Storage, CommandRequest, CommandResponse};

pub mod command_service;

// 从 Request 中得到 Response， 目前处理 HGET/HGETALL/HSET
pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Hget(param)) => param.execute(store),
        Some(RequestData::Hgetall(param)) => param.execute(store),
        Some(RequestData::Hset(param)) => param.execute(store),
        None => todo!(),
        _ => todo!(),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn service_should_works() {
        // 我们需要一个service，结构至少包含 storage
        // let service = Service::new(MemTable::default());
    }
}
