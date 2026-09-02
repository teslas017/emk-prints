use std::env;
#[derive(Clone)]
pub struct Config { pub host:String, pub port:u16, pub database_url:String, pub frontend_origin:String, pub jwt_secret:String, pub payment_webhook_secret:String }
impl Config {
 pub fn from_env()->Result<Self,String>{
  let required=|key:&str|env::var(key).map_err(|_|format!("missing {key}"));
  let jwt_secret=required("JWT_SECRET")?; if jwt_secret.len()<64{return Err("JWT_SECRET must contain at least 64 characters".into())}
  Ok(Self{host:env::var("HOST").unwrap_or_else(|_|"127.0.0.1".into()),port:env::var("PORT").unwrap_or_else(|_|"8080".into()).parse().map_err(|_|"invalid PORT")?,database_url:required("DATABASE_URL")?,frontend_origin:required("FRONTEND_ORIGIN")?,jwt_secret,payment_webhook_secret:required("PAYMENT_WEBHOOK_SECRET")?})
 }
}

