use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use grants_stack_api::{database, models, seed, utils};
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting server ...");
    HttpServer::new(|| {
        App::new()
            .service(seed_handler)
            .service(get_rounds_handler)
            .service(get_projects_handler)
            .service(get_votes_handler)
            .service(get_ipfs_handler)
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

#[derive(Deserialize)]
struct GetVotesQueryParams {
    project_id: Option<String>,
}
// an endpoint for getting votes
// use ?project_id=0x01,0x02,... to fetch votes of the project ids
// if project_id is not passed, return unfiltered votes
#[get("/votes")]
async fn get_votes_handler(query: web::Query<GetVotesQueryParams>) -> impl Responder {
    let pg = &mut utils::establish_pg_connection().unwrap();
    // if a user queries with 1 or more project_id
    if query.project_id.is_some() {
        let project_ids = query
            .project_id
            .clone()
            .unwrap()
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut votes: Vec<models::Vote> = Vec::new();
        for id in project_ids {
            let votes_of_project_id = database::get_votes_of_project_id(pg, &id).await;
            votes.extend(votes_of_project_id);
        }
        HttpResponse::Ok().json(votes)
    } else {
        let votes = database::get_votes(pg).await;
        HttpResponse::Ok().json(votes)
    }
}

#[derive(Deserialize)]
struct GetIPFSQueryParams {
    cid: Option<String>,
}
// an endpoint for relaying an ipfs query
// TODO: Investigate caching
#[get("/ipfs")]
async fn get_ipfs_handler(query: web::Query<GetIPFSQueryParams>) -> impl Responder {
    let cid = query.cid.clone();
    if cid.is_some() {
        let ipfs_data = utils::fetch_from_ipfs(&cid.unwrap()).await.unwrap();
        HttpResponse::Ok().json(ipfs_data)
    } else {
        HttpResponse::Ok().body("No")
    }
}
