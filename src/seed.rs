use diesel::PgConnection;
use gql_client::Client;

use crate::{database, utils};

pub async fn seed_chain_data(gql: &Client, pg: &mut PgConnection, chain_id: u16) {
    seed_programs(&gql, pg, chain_id).await;
    seed_rounds(&gql, pg, chain_id).await;
    // seed_project_meta_ptrs(gql, pg, chain_id).await;
    seed_projects(&gql, pg, chain_id).await;
    seed_votes(&gql, pg, chain_id).await;
    println!("done: {:?} data seeding", chain_id);
}

async fn seed_programs(gql: &Client, conn: &mut PgConnection, chain_id: u16) {
    let res = utils::r_query_programs(gql, "").await;
    let res = utils::add_program_chain_id(res, chain_id).await;

    database::new_programs(conn, res);
}

async fn seed_rounds(gql: &Client, conn: &mut PgConnection, chain_id: u16) {
    let res = utils::r_query_rounds(gql, "").await;
    let res = utils::add_round_chain_id(res, chain_id).await;

    database::new_rounds(conn, res);
}

async fn seed_projects(gql: &Client, conn: &mut PgConnection, chain_id: u16) {
    let res = utils::r_query_projects(gql, "").await;
    let res = utils::add_project_chain_id(res, chain_id).await;

    database::new_projects(conn, res);
}

async fn seed_votes(gql: &Client, conn: &mut PgConnection, chain_id: u16) {
    let res = utils::r_query_votes(gql, "").await;
    let res = utils::add_vote_chain_id(res, chain_id).await;
    // let res = utils::backfill_project_id(res).await;

    database::new_votes(conn, res);
}

async fn seed_project_meta_ptrs(gql: &Client, conn: &mut PgConnection, chain_id: u16) {
    let res = utils::r_query_project_meta_ptrs(gql, "").await;
    println!("res: {:?}", res);
    // let res = utils::add_project_meta_ptr_chain_id(res, chain_id).await;

    database::new_projects_meta_ptrs(conn, res);
}
