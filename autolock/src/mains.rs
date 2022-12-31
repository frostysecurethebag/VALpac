use reqwest::{Client, ClientBuilder, header::HeaderMap};
use native_tls::TlsConnector;
use serde::{Deserialize, Serialize};
use openssl::ssl::{SslContext, SslMethod, SslContextBuilder};
use std::collections::HashMap;
use serde_json::json;
use regex::Regex;

mod auth;





trait Purpose{
    fn authflow(&self);
}




// Create an SSL context.


#[derive(Debug, Serialize, Deserialize)]
struct Post {
  client_id: String,
  nonce: String,
  redirect_uri: String,
  response_type: String,
  scope: String
}


#[derive(Debug, Serialize, Deserialize)]
struct Info {
    access_token:String,
    token_id:String,
    time:String,
    entitlement_token:String,
    puuid:String,
}


// struct Res{
//     types:String,
//     response: {
//         width:String
//     }

// }
// struct Mod {
//     mode:String,

//     parameters:
// }


fn create_client() -> reqwest::Result<Client> {

    let force_ciphers = vec![
    "ECDHE-ECDSA-AES256-GCM-SHA384".to_string(),
    "ECDHE-ECDSA-AES128-GCM-SHA256".to_string(),
    "ECDHE-ECDSA-CHACHA20-POLY1305".to_string(),
    "ECDHE-RSA-AES128-GCM-SHA256".to_string(),
    "ECDHE-RSA-CHACHA20-POLY1305".to_string(),
    "ECDHE-RSA-AES128-SHA256".to_string(),
    "ECDHE-RSA-AES128-SHA".to_string(),
    "ECDHE-RSA-AES256-SHA".to_string(),
    "ECDHE-ECDSA-AES128-SHA256".to_string(),
    "ECDHE-ECDSA-AES128-SHA.to_string()".to_string(),
    "ECDHE-ECDSA-AES256-SHA.to_string()".to_string(),
    "ECDHE+AES128".to_string(),
    "ECDHE+AES256".to_string(),
    "ECDHE+3DES".to_string(),
    "RSA+AES128".to_string(),
    "RSA+AES256".to_string(),
    "RSA+3DES".to_string(),
    ];

    let mut context = SslContextBuilder::new(SslMethod::tls()).unwrap();

// Set the ciphers that you want to use.
    context.set_cipher_list(&(force_ciphers.join(":")));

    // use rust_tls::{ServerConfig, ClientConfig};

    // fn main() {
    //     // Set up the TLS 1.3 server config.
    //     let mut server_config = ServerConfig::new(rust_tls::NoClientAuth::new());
    //     server_config.ciphersuite = rust_tls::SupportedCipherSuite::TLS_AES_128_GCM_SHA256;
    
    //     // Set up the TLS 1.3 client config.
    //     let mut client_config = ClientConfig::new();
    //     client_config.ciphersuite = rust_tls::SupportedCipherSuite::TLS_AES_128_GCM_SHA256;
    
    //     // Use the server and client configs to set up your TLS 1.3 server and client.
    // }
    






    // Create a TLS connector with TLS 1.3 enabled
    let mut tls_builder = TlsConnector::builder()
    // tls_builder.min_version();
    // let tls_connector = tls_builder.build()?;
    .use_sni(false)
    .danger_accept_invalid_certs(true)
    .build()
    .unwrap();

    // Create a client with the TLS connector
    let client = Client::builder()
        .user_agent("RiotClient/60.0.6.4770705.4749685 rso-auth (Windows;10;;Professional, x64)")
        .cookie_store(true)
        // .use_preconfigured_tls(context)
        .build()
        .unwrap();

    Ok(client)
}

#[tokio::main]
async fn main(){
    // todo!()
    runner().await.unwrap();
}


async fn runner () -> Result<(), reqwest::Error>{


// // Use the context to create an SSL stream.
//     let stream = context.build();

// let mut headers = HeaderMap::new();
// headers.extend(
//     vec![
//         ("Content-Type", "application/json"),
//         ("Authorization", "Bearer abc123"),
//     ]
// );

    // ,
    // "User-Agent": Auth.RIOT_CLIENT_USER_AGENT,
    // "Accept": ", , */*",
    let data=Post{
        client_id: "play-valorant-web-prod".to_string(),
        nonce: "1".to_string(),
        redirect_uri: "https://playvalorant.com/opt_in".to_string(),
        response_type: "token id_token".to_string(),
        scope: "account openid".to_string(),
    };
    let client  = create_client().unwrap();
    // let client = Client::new();
    // client
    



    let rsp_4 = client
    .post("https://auth.riotgames.com/api/v1/authorization")
    .json(&data)
    .header("Content-Type", "application/json")
    .header("Accept", "application/json, text/plain, */*")
    // .headers(HeaderMap::from_iter(iter) headers)
    .send().await?;
    // println!("Get Fucked");
    // println!("{:#?}", &rsp_4);

    let rsp_4 = client
    .post("https://auth.riotgames.com/api/v1/authorization")
    .json(&data)
    .header("Content-Type", "application/json")
    .header("Accept", "application/json, text/plain, */*")
    // .headers(HeaderMap::from_iter(iter) headers)
    .send().await?;
    // println!("Get Fucked");

    let rsp_5 = client
    .put("https://auth.riotgames.com/api/v1/authorization")
    .json(&json!({
        "type": "auth", "username": "newacc69420", "password": "newacc69", "remember": true
    })

    )
    .header("Content-Type", "application/json")
    .header("Accept", "application/json, text/plain, */*")
    // .headers(HeaderMap::from_iter(iter) headers)
    .send().await?;
    // println!("Get Fucked");
    // println!("{:#?}", &rsp_4.json().await?);

    // let text = &rsp_5.text().await?;
    // println!("{}", text);

    let json: &serde_json::Value  = &rsp_5.json().await?;
    let pattern=&json["response"]["parameters"]["uri"].to_string();
    
    // println!("{:?}",&pattern);
    
    
    let re = Regex::new(r"access_token=((?:[a-zA-Z]|\d|\.|-|_)*).*id_token=((?:[a-zA-Z]|\d|\.|-|_)*).*expires_in=(\d*)").unwrap();
    let cap = re.captures(&pattern).unwrap();
    let mut nw_info:Info = Info { access_token: cap[1].to_string(),
        token_id:cap[2].to_string(),
        time: cap[3].to_string() ,
        entitlement_token: "".to_string(),
        puuid: "".to_string()
    };
    // println!("{:#?}",&nw_info.access_token);



    let client2  = create_client().unwrap();
    
    let rsp_5 = client2
    .post("https://entitlements.auth.riotgames.com/api/token/v1")
    .json(&json!({})

    )
    .header("Content-Type", "application/json")
    .bearer_auth(&nw_info.access_token)
    // .headers(HeaderMap::from_iter(iter) headers)
    .send().await?;

    let json: &serde_json::Value  = &rsp_5.json().await?;
    nw_info.entitlement_token=json["entitlements_token"].to_string().replace("\"", "");

    let client2  = create_client().unwrap();
    
    let rsp_5 = client2
    .post("https://auth.riotgames.com/userinfo")
    .json(&json!({})

    )
    .header("Content-Type", "application/json")
    .bearer_auth(&nw_info.access_token)
    // .headers(HeaderMap::from_iter(iter) headers)
    .send().await?;

    let json: &serde_json::Value  = &rsp_5.json().await?;
    nw_info.puuid=json["sub"].to_string().replace("\"", "");

    let client3  = create_client().unwrap();

    let rsp_6 = client3
    .get(format!("https://glz-ap-1.ap.a.pvp.net/pregame/v1/players/{}",&nw_info.puuid))
    // .json(&json!({}))
    .header("X-Riot-Entitlements-JWT".to_string(), &nw_info.entitlement_token.to_owned())
    .bearer_auth(&nw_info.access_token)
    // .headers(HeaderMap::from_iter(iter) headers)
    .send().await?;
    println!("{:?}",&nw_info);


    println!("{}",&nw_info.entitlement_token);

    let json: &serde_json::Value  = &rsp_6.json().await?;
    println!("{}",&json);
    let match_id=json["MatchID"].to_string();
    println!("{}",&match_id);



          //   if match_id=="null"{
      //     println!("Waiting For a Match");
      //   }else{
      //     client3
      //     .post(format!("https://glz-ap-1.ap.a.pvp.net/pregame/v1/matches/{}/select/a3bfb853-43b2-7238-a4f1-ad90e9e46bcc",&match_id))
      //     .json(&json!({}))
      //     .header("X-Riot-Entitlements-JWT", &complete_info.entitlement_token)
      //     .bearer_auth(&complete_info.access_token)
      //     .send().await?;
    
      //     client3
      //     .post(format!("https://glz-ap-1.ap.a.pvp.net/pregame/v1/matches/{}/lock/a3bfb853-43b2-7238-a4f1-ad90e9e46bcc",&match_id))
      //     .json(&json!({}))
      //     .header("X-Riot-Entitlements-JWT", &complete_info.entitlement_token)
      //     .bearer_auth(&complete_info.access_token)
      //     .send().await?;
      //     println!("Successfully Instalocked");
      //     break;
      //   }
    // client2
    // .post(format!("https://glz-ap-1.ap.a.pvp.net/pregame/v1/matches/{}/select/reyna",&match_id))
    // .json(&json!({}))
    // .header("X-Riot-Entitlements-JWT", &nw_info.entitlement_token)
    // .bearer_auth(&nw_info.access_token)
    // // .headers(HeaderMap::from_iter(iter) headers)
    // .send().await?;

    // client2
    // .post(format!("https://glz-ap-1.ap.a.pvp.net/pregame/v1/matches/{}/lock/reyna",&match_id))
    // .json(&json!({}))
    // .header("X-Riot-Entitlements-JWT", &nw_info.entitlement_token)
    // .bearer_auth(&nw_info.access_token)
    // // .headers(HeaderMap::from_iter(iter) headers)
    // .send().await?;


    // println!("{}",nw_info.puuid);









    // for cap in re.captures_iter(&pattern) {
    //     nw_info=Info{
    //         access_token: cap[1].to_string(),
    //         time:cap[3].to_string()
    //     };
    //     // println!("{} 0000 {} 1111 {} 2222 ", &cap[1], &cap[2], &cap[3]);
    // }
    // println!("{:?}",&nw_info);
    // let res=&re.find(pattern).unwrap();

    // match |&res|{

    // }
    // for i in rsp_4.cookies().into_iter(){
    //     println!("{:?}",i)
    // }


    Ok(())
}






// use reqwest;
// use native_tls;

// use std::collections::HashMap;
// use native_tls::{TlsConnector, Protocol};

// fn main() {
//     // Set up the TLS connector
//     let connector = TlsConnector::new().unwrap();
//     let connector = connector
//         .supported_protocols(&[Protocol::Tlsv10])
//         .unwrap();
//         connector.set_ciphers(":".join(FORCED_CIPHERS)).unwrap();

//     // Set up the request parameters
//     let url = "https://auth.riotgames.com/api/v1/authorization";
//     let mut headers = HashMap::new();
//     headers.insert("User-Agent", "my-rust-client/1.0");

//     // Make the request
//     let client = reqwest::Client::builder()
//         .danger_accept_invalid_certs(true) // accept invalid TLS certificates (not recommended in production)
//         .build()
//         .unwrap();
//     let res = client.get(url)
//         .headers(headers)
//         .send()
//         .unwrap();

//     // Check the status code
//     assert_eq!(res.status(), reqwest::StatusCode::OK);

//     // Read the response body
//     let body = res.text().unwrap();
//     println!("Response body: {}", body);
// }