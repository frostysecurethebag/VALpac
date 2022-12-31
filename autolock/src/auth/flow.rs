use std::{time,thread};

use reqwest;
use serde::{Serialize,Deserialize};
use serde_json::{json};
use regex::Regex;

use crate::auth::client;
use crate::auth::agents;

#[derive(Debug, Serialize, Deserialize)]
struct Post {
  client_id: String,
  nonce: String,
  redirect_uri: String,
  response_type: String,
  scope: String
}



#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    access_token:String,
    token_id:String,
    time:String,
    entitlement_token:String,
    puuid:String,
}

pub struct Auth;

impl  Auth{

    pub async fn authenticate(&self , username:&str, password:&str) -> Result<Info,reqwest::Error>{
        let client1=client::create_client().await?;

        let data=Post{
          client_id: "play-valorant-web-prod".to_string(),
          nonce: "1".to_string(),
          redirect_uri: "https://playvalorant.com/opt_in".to_string(),
          response_type: "token id_token".to_string(),
          scope: "account openid".to_string(),
      };

      client1
      .post("https://auth.riotgames.com/api/v1/authorization")
      .json(&data)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json, text/plain, */*")
      .send().await?;

  
      client1
      .post("https://auth.riotgames.com/api/v1/authorization")
      .json(&data)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json, text/plain, */*")
      .send().await?;

  
      let rsp_5 = client1
      .put("https://auth.riotgames.com/api/v1/authorization")
      .json(&json!({
          "type": "auth", "username": username, "password": password, "remember": true
      }))
      .header("Content-Type", "application/json")
      .header("Accept", "application/json, text/plain, */*")
      .send().await?;

      let json: &serde_json::Value  = &rsp_5.json().await?;
      let pattern=&json["response"]["parameters"]["uri"].to_string();
      
      let nw_info:Info = self.simplifier(&pattern).await;

      // print!("{:?}",nw_info);
      
  
      Ok(nw_info)
    }

    pub async fn get_entitlements(&self,mut nw_info:Info)-> Result<Info,reqwest::Error> {
      let client2  = client::create_client().await?;
      
      let rsp_5 = client2
      .post("https://entitlements.auth.riotgames.com/api/token/v1")
      .json(&json!({}))
      .header("Content-Type", "application/json")
      .bearer_auth(&nw_info.access_token)
      .send().await?;
  
      let json: &serde_json::Value  = &rsp_5.json().await?;
      nw_info.entitlement_token=json["entitlements_token"].to_string().replace("\"", "");
      let nw_info= self.get_user_info(nw_info);

      Ok(nw_info.await?)
    }

    pub async fn get_user_info(&self,mut nw_info:Info)->Result<Info,reqwest::Error>{
      let client2  = client::create_client().await?;
    
      let rsp_5 = client2
      .post("https://auth.riotgames.com/userinfo")
      .json(&json!({}))
      .header("Content-Type", "application/json")
      .bearer_auth(&nw_info.access_token)
      // .headers(HeaderMap::from_iter(iter) headers)
      .send().await?;

      let json: &serde_json::Value  = &rsp_5.json().await?;
      nw_info.puuid=json["sub"].to_string().replace("\"", "");

      Ok(nw_info)
    }


    pub async fn instalock(&self,complete_info:Info,agent_id:&str)-> Result<Info,reqwest::Error>{
      let delay = time::Duration::from_secs(3);
      let client3  = client::create_client().await?;

      loop{
        let rsp_6 = client3
        .get(format!("https://glz-ap-1.ap.a.pvp.net/pregame/v1/players/{}",&complete_info.puuid))
        .json(&json!({}))
        .header("X-Riot-Entitlements-JWT".to_string(), &complete_info.entitlement_token.to_owned())
        .bearer_auth(&complete_info.access_token)
        .send().await?;
        // println!("{:?}",&complete_info);
  
        let json: &serde_json::Value  = &rsp_6.json().await?;
        let match_id=json["MatchID"].to_string().replace("\"", "");

        match &match_id.as_str() {
          &"null" => println!("Waiting For a Match"),
          _ => {
                client3
                .post(format!("https://glz-ap-1.ap.a.pvp.net/pregame/v1/matches/{}/select/{}",&match_id,&agent_id))
                .json(&json!({}))
                .header("X-Riot-Entitlements-JWT", &complete_info.entitlement_token)
                .bearer_auth(&complete_info.access_token)
                .send().await?;
          
                client3
                .post(format!("https://glz-ap-1.ap.a.pvp.net/pregame/v1/matches/{}/lock/{}",&match_id,&agent_id))
                .json(&json!({}))
                .header("X-Riot-Entitlements-JWT", &complete_info.entitlement_token)
                .bearer_auth(&complete_info.access_token)
                .send().await?;
                println!("Successfully Instalocked");
                break;
              },
        }
        thread::sleep(delay);
      }
      
      Ok(complete_info)
    }

    pub async fn simplifier(&self,pattern:&String) -> Info{
      let re = Regex::new(r"access_token=((?:[a-zA-Z]|\d|\.|-|_)*).*id_token=((?:[a-zA-Z]|\d|\.|-|_)*).*expires_in=(\d*)").unwrap();
      let cap = re.captures(&pattern).unwrap();
      let nw_info:Info = Info { access_token: cap[1].to_string(),
          token_id:cap[2].to_string(),
          time: cap[3].to_string() ,
          entitlement_token: "".to_string(),
          puuid: "".to_string()
      };

      nw_info
    }

    pub async fn authflow(&self,username:&str,password:&str,agent:&str) ->Result<String,reqwest::Error>{
      let nw_info = self.authenticate(username,password).await?;
      let complete_info=self.get_entitlements(nw_info).await?;
      self.instalock(complete_info,agents::get_agentid(agent)).await?;
      Ok("Success".to_string())
    }
}
