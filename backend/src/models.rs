use chrono::{DateTime,Utc};use serde::{Deserialize,Serialize};use uuid::Uuid;use validator::Validate;
#[derive(Serialize,sqlx::FromRow)] pub struct Product{pub id:Uuid,pub name:String,pub slug:String,pub league:String,pub price_kes:Option<i32>,pub status:String,pub created_at:DateTime<Utc>,pub updated_at:DateTime<Utc>}
#[derive(Deserialize,Validate)] pub struct ProductInput{#[validate(length(min=2,max=120))]pub name:String,#[validate(length(min=2,max=120))]pub league:String,pub price_kes:Option<i32>,pub status:String}
#[derive(Deserialize,Validate)] pub struct LoginInput{#[validate(email)]pub email:String,#[validate(length(min=12,max=200))]pub password:String}
#[derive(Deserialize,Validate)] pub struct OrderInput{#[validate(length(min=2,max=120))]pub customer_name:String,#[validate(length(min=10,max=20))]pub phone:String,pub items:Vec<OrderItemInput>}
#[derive(Deserialize,Validate)] pub struct OrderItemInput{pub product_id:Uuid,#[validate(length(min=1,max=3))]pub size:String,#[validate(range(min=1,max=10))]pub quantity:i32}

