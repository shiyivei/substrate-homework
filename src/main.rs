use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use webserver::ThreadPool;

fn main() {
    //监听端口
    //bind函数返回一个Result枚举，使用模式匹配来处理可能遇到的错误
    let listener = match TcpListener::bind("127.0.0.1:3030") {
        Ok(listener) => listener,
        Err(_e) => panic!("failed to listen"),
    };

    //限定线程数量，防止dos攻击
    let pool = ThreadPool::new(4);

    //使用for循环遍历监听到的请求
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        //使用数量有限的线程处理请求
        pool.execute(|| handle_connection(stream));

        //多线程处理，如果对线程创建不加限制，在遭遇Dos攻击时会崩溃
        //thread::spawn(|| handle_connection(stream));

        //单线程处理
        //handle_connection(stream)
    }
    println!("shutting down")
}

//处理请求
fn handle_connection(mut stream: TcpStream) {
    //缓存，512字节
    let mut buffer = [0; 512];
    //将内容读到缓存中
    stream.read(&mut buffer).unwrap();

    //将缓存中的内容读为str
    // println!("Request:{}", String::from_utf8_lossy(&buffer[..]));

    //比对变量,b是字节字符串语法，可以将GET里面的文本转换为字节字符串
    let get = b"GET / HTTP/1.1\r\n";
    //模拟慢请求
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    //判断

    //使用元组匹配判断结果
    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        //休眠五秒
        thread::sleep(Duration::from_secs(5));
        //返回结果
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    //读取文件内容为str
    let contents = fs::read_to_string(file_name).unwrap();
    //格式化
    let response = format!("{}{}", status_line, contents);
    //转换为bytes
    stream.write(response.as_bytes()).unwrap();
    //直到全部输出
    stream.flush().unwrap();
}
