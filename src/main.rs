use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize, Deserialize};
use cedar_policy::{PolicySet, Authorizer, Entities, Context, EntityUid, Request, Decision};

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct EvaluatePayload {
    policies: Vec<String>,
    resources: Vec<String>,
    action: String,
    principal: String,
    context: Option<serde_json::Value>,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[post("/evaluate")]
async fn evaluate(payload: web::Json<EvaluatePayload>) -> impl Responder {
    let client: EntityUid = payload.principal.parse().expect("entity parse error");
    let endpoint: EntityUid = payload.resources[0].parse().expect("entity parse error");
    let action: EntityUid = payload.action.parse().expect("entity parse error");

    let request = match &payload.context {
        Some(context_value) => {
            let context = Context::from_json_value(context_value.clone(), None).unwrap();
            Request::new(Some(client), Some(action), Some(endpoint), context)
        },
        None => Request::new(Some(client), Some(action), Some(endpoint), Context::empty()),
    };



    let policy: PolicySet = payload.policies[0].parse().expect("policy parse error");

    let entities_json = r#"[]"#;
    let entities = Entities::from_json_str(entities_json, None).expect("entity parse error");

    let authorizer = Authorizer::new();
    let response = authorizer.is_authorized(&request, &policy, &entities);

    match response.decision() {
        Decision::Allow => {
            HttpResponse::Ok().json(Response { message: "Authorization decision: allow.".to_string() })
        }
        Decision::Deny => {
            HttpResponse::Ok().json(Response { message: "Authorization decision: deny.".to_string() })
        }
    }
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(healthcheck).service(evaluate).default_service(web::route().to(not_found)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
