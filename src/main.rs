use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, http::header::q};
use grants_stack_api::{database, models::{self, RoundMetaPtrItem, RoundProjectsMetaPtrItem}, seed, utils};
use serde::{Deserialize, Serialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting server ...");
    HttpServer::new(|| {
        App::new()
            .service(seed_handler)
            .service(get_round_data_handler)
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

#[derive(Clone, Deserialize, Debug)]
struct GetRoundDataQueryParams {
    round_id: String,
    round_meta_ptr: Option<bool>,
    projects_meta_ptr: Option<bool>,
}

#[derive(Deserialize,  Serialize, Debug)]
struct RoundData {
    round_meta_ptr: Option<RoundMetaPtrItem>,
    projects_meta_ptr: Option<RoundProjectsMetaPtrItem>,
}

// an endpoint for getting round data
// use ?round_id=0x01...?query={data, round_meta_ptr, voting_strategy, round_projects_meta_ptr, project_meta_ptr, round_projects, qf_votes}
#[get("/round")]
async fn get_round_data_handler(query: web::Query<GetRoundDataQueryParams>) -> impl Responder {
    let pg = &mut utils::establish_pg_connection().unwrap();

    if query.round_id.is_empty() {
        return HttpResponse::BadRequest().body("error: round_id is required");
    }

    // let round_id = query.round_id.clone();
    let query = query.clone();
    let round_id = &query.round_id;
    if round_id.is_empty() {
        return HttpResponse::BadRequest().body("error: round_id is required");
    }
    let mut round_data = RoundData {
        round_meta_ptr: None,
        projects_meta_ptr: None,
    };

    if query.round_meta_ptr.unwrap_or(false) {
        let round_meta_ptr = database::get_round_meta_ptr(pg, round_id.to_string()).await;
        round_data.round_meta_ptr = Some(round_meta_ptr[0].clone());
    }

    if query.projects_meta_ptr.unwrap_or(false) {
        let projects_meta_ptr = database::get_round_projects_meta_ptr(pg, round_id.to_string()).await;
        round_data.projects_meta_ptr = Some(projects_meta_ptr[0].clone());
    }

    // TODO: implement other query params

    HttpResponse::Ok().json(round_data)

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
