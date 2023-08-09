use crate::{
    command_request::RequestData, CommandRequest, CommandResponse, KvError, Storage, MemTable,
};
use std::sync::Arc;
use tracing::debug;

mod command_service;

/// 对Command的处理的抽象
pub trait CommandService {
    /// 处理Command，返回Response
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

/// Service 数据结构
pub struct Service<Store = MemTable> {
    inner: Arc<ServiceInner<Store>>,
}

impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

/// Service 内部数据结构
pub struct ServiceInner<Store> {
    store: Store,
}

impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self { inner: Arc::new(ServiceInner { store }) }
    }

    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        debug!("Got request: {:?}", cmd);
        let res = dispatch(cmd, &self.inner.store);
        debug!("Execute reponse: {:?}", res);
        res
    }
}

// 从Request中得到Response，目前处理HGET HGETALL HSET
pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Hget(param)) => param.execute(store),
        Some(RequestData::Hgetall(param)) => param.execute(store),
        Some(RequestData::Hset(param)) => param.execute(store),
        None => KvError::InvalidCommand("Reauest has no data".into()).into(),
        _ => KvError::Internal("Not implemented".into()).into(),
    }
}
