use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use grants_stack_api::{database, models, seed, utils};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting server ...");
    HttpServer::new(|| {
        App::new()
            .service(seed_handler)
            .service(get_round_handler)
            .service(get_project_handler)
            .service(get_ipfs_handler)
            .service(health_check_handler)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

/// Triggers seeding of the whole database
/// Takes a long time for chains with a lot of votes
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

// an endpoint for getting round data
// use ?round_id=0x01...&{data, round_meta_ptr, voting_strategy, round_projects_meta_ptr, round_projects, qf_votes}=true/false
// multiple params can be used at once
#[get("/round")]
async fn get_round_handler(query: web::Query<models::GetRoundDataQueryParams>) -> impl Responder {
    let mut res_data = models::RoundResponseData {
        data: None,
        round_meta_ptr: None,
        voting_strategy: None,
        projects_meta_ptr: None,
        round_projects: None,
        round_votes: None,
        round_summary: None,
    };

    let pg = &mut utils::establish_pg_connection().unwrap();

    if query.round_id.is_empty() {
        return HttpResponse::BadRequest().body("error: round_id is required");
    }

    let query = query.clone();

    let round_id = &query.round_id;
    if round_id.is_empty() {
        return HttpResponse::BadRequest().body("error: round_id is required");
    }

    if query.data.unwrap_or(false) {
        let round_data = database::get_round_data(pg, round_id.to_string()).await;
        if !round_data.is_empty() {
            res_data.data = Some(round_data[0].clone());
        }
    }

    if query.round_meta_ptr.unwrap_or(false) {
        let round_meta_ptr = database::get_round_meta_ptr(pg, round_id.to_string()).await;
        if !round_meta_ptr.is_empty() {
            res_data.round_meta_ptr = Some(round_meta_ptr[0].clone());
        }
    }

    if query.voting_strategy.unwrap_or(false) {
        let voting_strategy = database::get_round_voting_strategy(pg, round_id.to_string()).await;
        if !voting_strategy.is_empty() {
            res_data.voting_strategy = Some(voting_strategy[0].clone());
        }
    }

    if query.projects_meta_ptr.unwrap_or(false) {
        let projects_meta_ptr =
            database::get_round_projects_meta_ptr(pg, round_id.to_string()).await;
        if !projects_meta_ptr.is_empty() {
            res_data.projects_meta_ptr = Some(projects_meta_ptr[0].clone());
        }
    }

    if query.round_projects.unwrap_or(false) {
        let round_projects = database::get_round_projects(pg, round_id.to_string()).await;
        if !round_projects.is_empty() {
            res_data.round_projects = Some(round_projects);
        }
    }

    if query.round_votes.unwrap_or(false) {
        let round_votes = database::get_round_votes(pg, round_id.to_string()).await;
        if !round_votes.is_empty() {
            res_data.round_votes = Some(round_votes);
        }
    }

    if query.round_summary.unwrap_or(false) {
        let round_summary = utils::summarize_round(pg, round_id.to_string()).await;
        res_data.round_summary = Some(round_summary);
    }

    HttpResponse::Ok().json(res_data)
}

/// an endpoint for getting project data
/// use ?project_id=0x01...&{data, project_meta_ptr, project_votes}=true/false
/// multiple params can be used at the same time
#[get("/project")]
async fn get_project_handler(
    query: web::Query<models::GetProjectDataQueryParams>,
) -> impl Responder {
    let mut res_data = models::ProjectResponseData {
        data: None,
        project_meta_ptr: None,
        project_votes: None,
        project_summary: None,
    };

    let pg = &mut utils::establish_pg_connection().unwrap();

    if query.project_id.is_empty() {
        return HttpResponse::BadRequest().body("error: project_id is required");
    }

    let query = query.clone();

    let project_id = &query.project_id;

    if query.data.unwrap_or(false) {
        let project_data = database::get_project_data(pg, project_id.to_string()).await;
        if !project_data.is_empty() {
            res_data.data = Some(project_data[0].clone());
        }
    }

    if query.project_meta_ptr.unwrap_or(false) {
        let project_meta_ptr = database::get_project_meta_ptr(pg, project_id.to_string()).await;
        if !project_meta_ptr.is_empty() {
            res_data.project_meta_ptr = Some(project_meta_ptr[0].clone());
        }
    }

    if query.project_votes.unwrap_or(false) {
        let project_votes = database::get_project_votes(pg, project_id.to_string()).await;
        if !project_votes.is_empty() {
            res_data.project_votes = Some(project_votes);
        }
    }

    if query.project_summary.unwrap_or(false) {
        let project_summary = utils::summarize_project(pg, project_id.to_string()).await;
        res_data.project_summary = Some(project_summary);
        // TODO: check if empty
    }

    HttpResponse::Ok().json(res_data)
}

// an endpoint for relaying an ipfs query
// TODO: Investigate caching
#[get("/ipfs")]
async fn get_ipfs_handler(query: web::Query<models::GetIPFSQueryParams>) -> impl Responder {
    let cid = query.cid.clone();
    if cid.is_some() {
        let ipfs_data = utils::fetch_from_ipfs(&cid.unwrap()).await.unwrap();
        HttpResponse::Ok().json(ipfs_data)
    } else {
        HttpResponse::Ok().body("No")
    }
}

/// Health check endpoint
/// Contacts the database and resolves to 200 if db can be reached,
/// 502 if database is not reached
#[get("/health")]
async fn health_check_handler() -> impl Responder {
    match utils::establish_pg_connection() {
        Ok(_) => HttpResponse::Ok().json("healthy!"),
        Err(e) => HttpResponse::BadGateway().body(e.to_string()),
    }
}
