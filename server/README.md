# TODO: 待添加文档

## 遇到的问题集合

### 想写一个 In memory session manager 中间件
难点在于得开一个异步线程去定时处理过期的key, 又得保证中间件能线程安全的存取key
你得使用 
- [Dashmap](https://crates.io/crates/dashmap): 一个并发Hashmap
- [AtomicRefCell](https://crates.io/crates/atomic_refcell): 一个线程安全的 RefCell
