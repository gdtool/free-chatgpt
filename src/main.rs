use actix_web::{error, get, post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use chatgpt::{Chatgpt, ChatgptParams};
use serde_json::json;
use std::{sync::Arc, vec};
use dotenv::dotenv;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("ok!")
}

#[post("/api")]
async fn ask(chatgpt_params: web::Json<ChatgptParams>) -> HttpResponse {
    let mut chatgpt_params = chatgpt_params.into_inner();
    if chatgpt_params.top_p.is_none() {
        chatgpt_params.top_p = Some(1);
    }
    if chatgpt_params.model.is_none() {
        chatgpt_params.model = Some(String::from("openai:gpt-3.5-turbo"));
    }
    if chatgpt_params.temperature.is_none() {
        chatgpt_params.temperature = Some(1.0);
    }
    if chatgpt_params.top_p.is_none() {
        chatgpt_params.top_p = Some(1);
    }
    if chatgpt_params.max_tokens.is_none() || chatgpt_params.max_tokens.unwrap() > 256 {
        chatgpt_params.max_tokens = Some(256);
    }
    if chatgpt_params.frequency_penalty.is_none() {
        chatgpt_params.frequency_penalty = Some(0);
    }
    if chatgpt_params.presence_penalty.is_none() {
        chatgpt_params.presence_penalty = Some(0);
    }
    if chatgpt_params.stop_sequences.is_none() {
        chatgpt_params.stop_sequences = Some(vec![]);
    }
    let chatgpt = Arc::new(Chatgpt::new().unwrap());
    match chatgpt.ask(chatgpt_params).await {
        Ok(res) => HttpResponse::Ok().json(json!({
            "code": 200,
            "msg": "success",
            "data": res,
        })),
        Err(err) => HttpResponse::Ok().json(json!({
            "code": 500,
            "msg": format!("error: {:?}", err),
            "data": ""
        })),
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    let _ = HttpServer::new(move || {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                let error = json!({
                    "code": 1000,
                    "msg": err.to_string(),
                    "data": ""
                });
                error::InternalError::from_response(err, HttpResponse::Ok().json(error)).into()
            });
        App::new().app_data(json_config).service(index).service(ask)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;
    Ok(())
}
