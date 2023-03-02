use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use grants_stack_api::{database, seed, utils};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting server ...");
    HttpServer::new(|| {
        App::new()
            .service(seed_handler)
            .service(get_rounds_handler)
            .service(get_projects_handler)
            .service(get_votes_handler)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

// an endpoint to trigger seeding
#[get("/seed/{chain_id}")]
async fn seed_handler(chain_id: web::Path<u16>) -> impl Responder {
    let chain_id = chain_id.into_inner();
    if !utils::check_chain_id(chain_id) {
        return HttpResponse::BadRequest().body("error: chain id not supported");
    }
    let pg = &mut utils::establish_pg_connection().unwrap();
    let gql = utils::establish_gql_client(chain_id);
    seed::seed_chain_data(&gql, pg, chain_id).await;
    HttpResponse::Ok().body("done: data seeding")
}

// an endpoint for getting all rounds
#[get("/rounds")]
async fn get_rounds_handler() -> impl Responder {
    let pg = &mut utils::establish_pg_connection().unwrap();
    let rounds = database::get_rounds(pg).await;
    HttpResponse::Ok().json(rounds)
}

// an endpoint for getting all projects
#[get("/projects")]
async fn get_projects_handler() -> impl Responder {
    let pg = &mut utils::establish_pg_connection().unwrap();
    let projects = database::get_projects(pg).await;
    HttpResponse::Ok().json(projects)
}

// an endpoint for getting all votes
#[get("/votes")]
async fn get_votes_handler() -> impl Responder {
    let pg = &mut utils::establish_pg_connection().unwrap();
    let votes = database::get_votes(pg).await;
    HttpResponse::Ok().json(votes)
}
