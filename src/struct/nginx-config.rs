// /**
// * @desc parser config struct
// * @reference Nginx document
// */
#[derive(Debug)]
// Main Module,TODO 如果写入文件解析需要转为Option<>
struct MainModule {
    user: String,
    // TODO auto 1 2 3 4 5 6
    worker_processes: u32,
    // event: EventModule,
    // error_log: String,
    // pid: String,
    // worker_rlimit_nofile: u32,
    // http: HttpModule,
}

// Event Module
struct EventModule {
    worker_connections: u32,
}

// Http Module
struct HttpModule {
    include: String,
    default_type: String,
    log_format: String,
    server_name_hash_bucket_size: u32,
    client_header_buffer_size: String,
    sendfile: String,
    // on | off
    gzip: String,
    gzip_comp_level: u32,
    gzip_buffers: u32,
    // text/plain  text/css application/javascript application/x-javascript text/xml application/xml application/xml+rss text/javascript;
    // text/plain  text/css application/javascript application/x-javascript text/xml application/xml application/xml+rss text/javascript;
    gzip_types: String,

    //TODO {1.1}
    gzip_http_version: String,
    keepalive_timeout: String,
    server: ServerModule,
}

// Server Module
struct ServerModule {
    listen: u32,
    location: Vec<LocationModule>,
}

// Location Module
struct LocationModule {
    root: String,
    index: String,
}

// Upstream Module

// * @TODO need `conf` file translate => `json` struct file

fn main() {
    let config = MainModule {
        user: String::from("www www"),
        worker_processes: 2,
        // error_log: String::from("/usr/local/rustic/logs/error.log cert"), //log path and level
        // pid: String::from("/usr/local/rustic/logs/rustic.pid"), // pid path
        // worker_rlimit_nofile: 65535, //指定此进程可以打开的最大文件描述符的值。
        // http: HttpModule {
        //     include: String::from("dsa"),
        //     default_type: "".to_string(),
        //     log_format: "".to_string(),
        //     server_name_hash_bucket_size: 0,
        //     client_header_buffer_size: "".to_string(),
        //     sendfile: "".to_string(),
        //     gzip: String::from("s"),
        //     gzip_comp_level: 0,
        //     gzip_buffers: 0,
        //     gzip_types: "".to_string(),
        //     gzip_http_version: "".to_string(),
        //     keepalive_timeout: "".to_string(),
        //     server: ServerModule {
        //         listen: 0,
        //         location: vec![]
        //     }
        // },
        // event: EventModule { worker_connections: 0 }
    };
    show_test();
    println!("==>{:#?}", config)
}

#[derive(Debug)] //TODO 干嘛的？
// <'a>是干嘛的？TODO
struct Person {
    name: String,
    age: u8,
}

fn show_test() {
    let name = "JoGel";
    let age = 26;
    let peter = Person {
        name: String::from(name),
        age,
    };

    //:#? 是干嘛的
    println!("{:#?}", peter);
}
