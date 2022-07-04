# 1. 运行

```
cargo run
```

# 2. 测试

## 1.在浏览器访问（推荐火狐）

```
http://127.0.0.1:3030/
```

输出

```
Hello SHIYIVEI

This is Rust
```

## 2.在浏览器访问（推荐火狐）

```
http://127.0.0.1:3030/sleep
```

延迟5s打印

```
Hello SHIYIVEI

This is Rust
```

## 3.访问其他网址

```
http://127.0.0.1:3030/s
```

得到错误信息

```
Ooops!

Sorry,I don't known what you are asking for
```

## 4.在终端查看输出

执行完会自行关闭

```
Worker 0 got a job; executing
Worker 1 got a job; executing
Worker 2 got a job; executing
Worker 3 got a job; executing
shutting down
Sending terminate message to all workers
shutting down all workers
shutting down worker 0
Worker 1 was told to terminate
Worker 2 was told to terminate
Worker 3 was told to terminate
...
```

