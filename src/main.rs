use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, http::header::q};
use grants_stack_api::{database, models::{self, RoundMetaPtrItem, RoundProjectsMetaPtrItem, RoundItem, VotingStrategyItem, ProjectItem, QfVoteItem}, seed, utils};
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
    data: Option<bool>,
    round_meta_ptr: Option<bool>,
    voting_strategy: Option<bool>,
    projects_meta_ptr: Option<bool>,
    round_projects: Option<bool>,
    round_votes: Option<bool>,
}

#[derive(Deserialize,  Serialize, Debug)]
struct RoundResponseData {
    data: Option<RoundItem>,
    round_meta_ptr: Option<RoundMetaPtrItem>,
    voting_strategy: Option<VotingStrategyItem>,
    projects_meta_ptr: Option<RoundProjectsMetaPtrItem>,
    round_projects: Option<Vec<ProjectItem>>,
    round_votes: Option<Vec<QfVoteItem>>,
}

// an endpoint for getting round data
// use ?round_id=0x01...?query={data, round_meta_ptr, voting_strategy, round_projects_meta_ptr, round_projects, qf_votes}
#[get("/round")]
async fn get_round_data_handler(query: web::Query<GetRoundDataQueryParams>) -> impl Responder {
    let mut res_data = RoundResponseData {
        data: None,
        round_meta_ptr: None,
        voting_strategy: None,
        projects_meta_ptr: None,
        round_projects: None,
        round_votes: None,
    };
    
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


    if query.data.unwrap_or(false) {
        let round_data = database::get_round_data(pg, round_id.to_string()).await;
        res_data.data = Some(round_data[0].clone());
    }

    if query.round_meta_ptr.unwrap_or(false) {
        let round_meta_ptr = database::get_round_meta_ptr(pg, round_id.to_string()).await;
        res_data.round_meta_ptr = Some(round_meta_ptr[0].clone());
    }

    if query.voting_strategy.unwrap_or(false) {
        let voting_strategy = database::get_round_voting_strategy(pg, round_id.to_string()).await;
        res_data.voting_strategy = Some(voting_strategy[0].clone());
    }

    if query.projects_meta_ptr.unwrap_or(false) {
        let projects_meta_ptr = database::get_round_projects_meta_ptr(pg, round_id.to_string()).await;
        res_data.projects_meta_ptr = Some(projects_meta_ptr[0].clone());
    }

    if query.round_projects.unwrap_or(false) {
        let round_projects = database::get_round_projects(pg, round_id.to_string()).await;
        res_data.round_projects = Some(round_projects);
    }

    if query.round_votes.unwrap_or(false) {
        let round_votes = database::get_round_votes(pg, round_id.to_string()).await;
        res_data.round_votes = Some(round_votes);
    }

    HttpResponse::Ok().json(res_data)

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
