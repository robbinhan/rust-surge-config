extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;


use std::fs::File;
use serde_json::Error;


#[derive(Serialize, Deserialize)]
struct SSConfig {
    localAuthPassword: String,
    configs: Vec<Config>,
}
#[derive(Serialize, Deserialize)]
struct Config {
    remarks: String,
    server: String,
    server_port: u32,
    method: String,
    password: String,
}

fn main() -> Result<(), Error> {

    let filename = "ss.json";

    let f = File::open(filename).expect("file not found");

    let v: SSConfig = serde_json::from_reader(f)?;

    let mut proxys_string  = String::new();
    let mut proxy_group_string = String::new();

    proxy_group_string = "Shadowsocks = select,".to_string();
    
    for x in v.configs {
        let proxy_string  = format!("{} = custom, {}, {}, {}, {}, http://cat-cdn.oss-cn-shenzhen.aliyuncs.com/SSEncrypt.module \n", x.remarks, x.server, x.server_port, x.method, x.password);
    
        proxys_string += &proxy_string;
        proxy_group_string += &(x.remarks + ",");
    }

    println!("[Proxy]\n{}", proxys_string); 
    println!("[Proxy Group]\n{}", proxy_group_string); 

    Ok(())
}
