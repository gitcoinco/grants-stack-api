use std::collections::HashMap;

use diesel::PgConnection;
use gql_client::Client;

use crate::{
    database,
    models::TokenPrice,
    utils::{self, query_token_prices_24hr},
};

pub async fn seed_chain_data(gql: &Client, pg: &mut PgConnection, chain_id: u16) {
    let rounds = utils::r_query_rounds(gql, "", chain_id).await;
    let round_meta_ptrs = utils::r_query_round_meta_ptr(gql, "", chain_id).await;
    let voting_strategies = utils::r_query_voting_strategies(gql, "", chain_id).await;
    let round_projects_meta_ptrs = utils::r_query_round_projects_meta_ptrs(gql, "", chain_id).await;
    let project_meta_ptrs = utils::r_query_project_meta_ptrs(gql, "", chain_id).await;
    let round_projects = utils::r_query_round_projects(gql, "", chain_id).await;
    let qf_votes = utils::r_query_qf_votes(gql, "", chain_id).await;
    // get all unique token addresses
    let mut token_addresses: Vec<String> = Vec::new();
    for vote in qf_votes.iter() {
        if !token_addresses.contains(&vote.token) {
            token_addresses.push(vote.token.clone());
        }
    }

    let mut token_prices: Vec<TokenPrice> = Vec::new();

    // hashmap of token address to token prices
    let mut token_prices_map: HashMap<String, Vec<TokenPrice>> = HashMap::new();

    for token_address in token_addresses.iter() {
        let token_price = query_token_prices_24hr(
            token_address.to_string(),
            chain_id,
            qf_votes.first().unwrap().createdAt.clone(),
            qf_votes.last().unwrap().createdAt.clone(),
        )
        .await;
        print!("{:?}", token_price);
        token_prices_map.insert(token_address.to_string(), token_price);
    }

    // for each token addresses prices, upload to db
    for (token_address, _) in token_prices_map.iter() {
        database::new_token_prices(
            pg,
            token_prices_map.get(token_address).unwrap().to_vec(),
            token_address.to_string(),
            chain_id.to_string(),
        );
    }

    database::new_rounds(pg, rounds);
    database::new_round_meta_ptrs(pg, round_meta_ptrs);
    database::new_voting_strategies(pg, voting_strategies);
    database::new_round_projects_meta_ptrs(pg, round_projects_meta_ptrs);
    database::new_project_meta_ptrs(pg, project_meta_ptrs);
    database::new_projects(pg, round_projects);
    database::new_qf_votes(pg, qf_votes);
}
