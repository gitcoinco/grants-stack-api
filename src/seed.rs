use diesel::PgConnection;
use gql_client::Client;

use crate::{utils, database};

pub async fn seed_chain_data(gql: &Client, pg: &mut PgConnection, chain_id: u16) {

    let rounds = utils::r_query_rounds(gql, "", chain_id).await;
    let round_meta_ptrs = utils::r_query_round_meta_ptr(gql, "", chain_id).await;
    let voting_strategies = utils::r_query_voting_strategies(gql, "", chain_id).await;
    let round_projects_meta_ptrs = utils::r_query_round_projects_meta_ptrs1(gql, "", chain_id).await;
    let project_meta_ptrs = utils::r_query_project_meta_ptrs(gql, "", chain_id).await;
    let round_projects = utils::r_query_round_projects(gql, "", chain_id).await;
    let qf_votes = utils::r_query_qf_votes(gql, "", chain_id).await;

    // // // print round meta ptrs
    // println!("round_meta_ptrs: {:?}", round_meta_ptrs);
    // // print project meta ptrs
    // println!("project_meta_ptrs: {:?}", project_meta_ptrs);
    // // print round projects meta ptrs
    // println!("round_projects_meta_ptrs: {:?}", round_projects_meta_ptrs);

    // // print voting strategies
    // println!("voting_strategies: {:?}", voting_strategies);
    // // print round projects
    // println!("round_projects: {:?}", round_projects);
    // // print qf votes
    // println!("qf_votes: {:?}", qf_votes);
    // // print rounds
    // println!("rounds: {:?}", rounds);

    // coerce voting strategies into a Vec<VotingStrategy>
    

    database::new_rounds1(pg, rounds);
    database::new_round_meta_ptrs(pg, round_meta_ptrs);
    database::new_voting_strategies(pg, voting_strategies);
    database::new_round_projects_meta_ptrs(pg, round_projects_meta_ptrs);
    database::new_project_meta_ptrs1(pg, project_meta_ptrs);
    database::new_projects1(pg, round_projects);
    database::new_qf_votes1(pg, qf_votes);
}