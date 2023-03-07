use diesel::PgConnection;
use gql_client::Client;

use crate::{utils, database};

pub async fn seed_chain_data(gql: &Client, pg: &mut PgConnection, chain_id: u16) {

    let rounds = utils::r_query_rounds(gql, "", chain_id).await;
    let round_meta_ptrs = utils::r_query_round_meta_ptr(gql, "", chain_id).await;
    let voting_strategies = utils::r_query_voting_strategies(gql, "", chain_id).await;
    let round_projects_meta_ptrs = utils::r_query_round_projects_meta_ptrs(gql, "", chain_id).await;
    let project_meta_ptrs = utils::r_query_project_meta_ptrs(gql, "", chain_id).await;
    let round_projects = utils::r_query_round_projects(gql, "", chain_id).await;
    let qf_votes = utils::r_query_qf_votes(gql, "", chain_id).await;
    
    database::new_rounds(pg, rounds);
    database::new_round_meta_ptrs(pg, round_meta_ptrs);
    database::new_voting_strategies(pg, voting_strategies);
    database::new_round_projects_meta_ptrs(pg, round_projects_meta_ptrs);
    database::new_project_meta_ptrs(pg, project_meta_ptrs);
    database::new_projects(pg, round_projects);
    database::new_qf_votes(pg, qf_votes);
}