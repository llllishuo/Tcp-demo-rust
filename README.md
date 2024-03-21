# Tcp-demo-rust

这是一个点对点通信的TCP服务器

## 运行

### 服务端
``` cargo run --bin server```
### 客户端
``` cargo run --bin client```

| 你可以开启多个client, 但是同一时间服务端只能与一个客户端通信, 需要按顺序关闭才可以进行下一个客户端的通信

## 简单说明

### cliend_vec
使用```Arc<Mutex<HashMap<String, TcpStream>>>```智能指针互斥锁的方式保存哈希表,以避免进程通信的问题.

```Arc::clone```Move操作

```cliend_vec.lock().unwrap().insert(stream_addr.to_string(), stream.try_clone().unwrap());```获取锁以添加键值对


## 感谢

感谢开源作者"thepacketgeek"的[文章](https://github.com/thepacketgeek/rust-tcpstream-demo/tree/master/raw#bufread-and-bufreader)

