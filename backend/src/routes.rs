use actix_web::{HttpRequest,HttpResponse,web};
use serde::Deserialize;
use subtle::ConstantTimeEq;
use validator::Validate;
use crate::{error::ApiError,models::{LoginInput,OrderInput,OrderStatusInput,Product,ProductInput,TrackingView},state::AppState};

pub fn configure(cfg:&mut web::ServiceConfig){cfg
 .route("/health",web::get().to(health)).route("/v1/products",web::get().to(products))
 .route("/v1/orders",web::post().to(create_order)).route("/v1/orders/track/{tracking}",web::get().to(track_order))
 .route("/v1/admin/login",web::post().to(login)).route("/v1/admin/products",web::post().to(create_product))
 .route("/v1/admin/orders/{id}/status",web::patch().to(update_order_status))
 .route("/v1/payments/webhook",web::post().to(payment_webhook));}
async fn health()->HttpResponse{HttpResponse::Ok().json(serde_json::json!({"status":"ok"}))}
async fn products(state:web::Data<AppState>)->Result<HttpResponse,ApiError>{let rows=sqlx::query_as::<_,Product>("SELECT id,name,slug,league,price_kes,status,created_at,updated_at FROM products WHERE status <> 'draft' ORDER BY name").fetch_all(&state.pool).await.map_err(|_|ApiError::Internal)?;Ok(HttpResponse::Ok().json(rows))}

async fn create_order(state:web::Data<AppState>,body:web::Json<OrderInput>)->Result<HttpResponse,ApiError>{
 body.validate().map_err(|_|ApiError::Validation)?;if body.items.is_empty(){return Err(ApiError::Validation)}
 let id=uuid::Uuid::new_v4();let short=id.simple().to_string().to_uppercase();let order_number=format!("EMK-{}",&short[..8]);let tracking_number=format!("TRK-{}",&short[8..18]);
 let mut tx=state.pool.begin().await.map_err(|_|ApiError::Internal)?;
 sqlx::query("INSERT INTO orders(id,order_number,tracking_number,customer_name,phone,status) VALUES($1,$2,$3,$4,$5,'payment_pending')").bind(id).bind(&order_number).bind(&tracking_number).bind(body.customer_name.trim()).bind(body.phone.trim()).execute(&mut *tx).await.map_err(|_|ApiError::Internal)?;
 for item in &body.items{
  if !matches!(item.size.as_str(),"S"|"M"|"L"|"XL")||!matches!(item.kit_type.as_str(),"home"|"away"|"third"){return Err(ApiError::Validation)}
  let row:Option<(uuid::Uuid,i32)>=sqlx::query_as("UPDATE product_variants v SET reserved_stock=reserved_stock+$1 FROM products p WHERE v.product_id=p.id AND p.id=$2 AND v.kit_type=$3 AND v.size=$4 AND p.status='active' AND v.stock-v.reserved_stock >= $1 RETURNING v.id,p.price_kes").bind(item.quantity).bind(item.product_id).bind(&item.kit_type).bind(&item.size).fetch_optional(&mut *tx).await.map_err(|_|ApiError::Internal)?;
  let (variant_id,price)=row.ok_or(ApiError::Validation)?;
  sqlx::query("INSERT INTO order_items(order_id,product_variant_id,quantity,unit_price_kes) VALUES($1,$2,$3,$4)").bind(id).bind(variant_id).bind(item.quantity).bind(price).execute(&mut *tx).await.map_err(|_|ApiError::Internal)?;
 }
 tx.commit().await.map_err(|_|ApiError::Internal)?;
 Ok(HttpResponse::Created().json(serde_json::json!({"order_id":id,"order_number":order_number,"tracking_number":tracking_number,"status":"payment_pending","reservation_minutes":15})))
}

#[derive(Deserialize)]struct TrackQuery{phone_last4:String}
async fn track_order(state:web::Data<AppState>,path:web::Path<String>,query:web::Query<TrackQuery>)->Result<HttpResponse,ApiError>{
 if query.phone_last4.len()!=4||!query.phone_last4.chars().all(|c|c.is_ascii_digit()){return Err(ApiError::Validation)}
 let row=sqlx::query_as::<_,TrackingView>("SELECT order_number,tracking_number,status,created_at,updated_at FROM orders WHERE tracking_number=$1 AND right(regexp_replace(phone,'\\D','','g'),4)=$2").bind(path.into_inner().trim().to_uppercase()).bind(&query.phone_last4).fetch_optional(&state.pool).await.map_err(|_|ApiError::Internal)?;
 row.map(|x|HttpResponse::Ok().json(x)).ok_or(ApiError::NotFound)
}
async fn login(body:web::Json<LoginInput>)->Result<HttpResponse,ApiError>{body.validate().map_err(|_|ApiError::Validation)?;Err(ApiError::Unauthorized)}
async fn create_product(_req:HttpRequest,_state:web::Data<AppState>,body:web::Json<ProductInput>)->Result<HttpResponse,ApiError>{body.validate().map_err(|_|ApiError::Validation)?;Err(ApiError::Unauthorized)}
async fn update_order_status(_req:HttpRequest,_state:web::Data<AppState>,_path:web::Path<uuid::Uuid>,body:web::Json<OrderStatusInput>)->Result<HttpResponse,ApiError>{body.validate().map_err(|_|ApiError::Validation)?;Err(ApiError::Unauthorized)}

#[derive(Deserialize)]struct PaymentNotice{provider_reference:String,payment_status:String}
async fn payment_webhook(req:HttpRequest,state:web::Data<AppState>,body:web::Json<PaymentNotice>)->Result<HttpResponse,ApiError>{
 let supplied=req.headers().get("x-emk-webhook-secret").and_then(|v|v.to_str().ok()).unwrap_or("");
 if supplied.as_bytes().ct_eq(state.config.payment_webhook_secret.as_bytes()).unwrap_u8()!=1{return Err(ApiError::Unauthorized)}
 if body.payment_status!="COMPLETED"{return Ok(HttpResponse::Accepted().finish())}
 let mut tx=state.pool.begin().await.map_err(|_|ApiError::Internal)?;
 let order:Option<(uuid::Uuid,String,String)>=sqlx::query_as("SELECT id,order_number,tracking_number FROM orders WHERE provider_reference=$1 AND payment_status='pending' FOR UPDATE").bind(&body.provider_reference).fetch_optional(&mut *tx).await.map_err(|_|ApiError::Internal)?;
 let Some((order_id,order_number,tracking_number))=order else{return Ok(HttpResponse::Ok().finish())};
 sqlx::query("UPDATE product_variants v SET stock=stock-i.quantity,reserved_stock=reserved_stock-i.quantity FROM order_items i WHERE i.order_id=$1 AND i.product_variant_id=v.id").bind(order_id).execute(&mut *tx).await.map_err(|_|ApiError::Internal)?;
 sqlx::query("UPDATE orders SET payment_status='completed',status='paid',paid_at=now(),updated_at=now() WHERE id=$1").bind(order_id).execute(&mut *tx).await.map_err(|_|ApiError::Internal)?;
 tx.commit().await.map_err(|_|ApiError::Internal)?;notify_owner(&state,&order_number,&tracking_number).await;Ok(HttpResponse::Ok().finish())
}
async fn notify_owner(state:&AppState,order:&str,tracking:&str){
 let url=format!("https://graph.facebook.com/v23.0/{}/messages",state.config.whatsapp_phone_number_id);
 let payload=serde_json::json!({"messaging_product":"whatsapp","to":state.config.owner_whatsapp,"type":"template","template":{"name":"new_paid_order","language":{"code":"en"},"components":[{"type":"body","parameters":[{"type":"text","text":order},{"type":"text","text":tracking}]}]}});
 if let Err(e)=reqwest::Client::new().post(url).bearer_auth(&state.config.whatsapp_token).json(&payload).send().await{tracing::error!(error=%e,order=%order,"WhatsApp order notification failed")}
}
