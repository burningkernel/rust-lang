mod memory;

pub use memory::MemTable;

use crate::{KvError, Kvpair, Value};

/// 对存储的抽象，我们不关心数据存储在什么地方，需要定义外界如何和存储打交道
pub trait Storage {
    /// 从一个HashTable中获取一个key的value
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 从一个HashTable里设置要给key的value， 返回旧的value
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    /// 查看HashTable中是否有key
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    /// 从HashTable中删除一个key
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 遍历HashTable，返回所有的kv pair
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    /// 遍历HashTable，返回kv pair 的Iterator
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}
