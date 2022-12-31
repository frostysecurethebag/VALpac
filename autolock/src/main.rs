mod auth;


use std::{env::{self}};

use crate::auth::flow;

#[tokio::main]
async fn main(){
    let args: Vec<String> = env::args().collect();
    let acc=get_acc().unwrap();
    let acc: Vec<&str> = acc.split(";").collect();

    flow::Auth.authflow(acc[0],acc[1],args[1].as_str()).await.unwrap();
}


fn get_acc()-> Result<std::string::String,std::io::Error>{
    let has=std::fs::read_to_string("account.txt").unwrap();
    Ok(has)
}