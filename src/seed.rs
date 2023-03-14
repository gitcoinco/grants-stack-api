use diesel::PgConnection;
use gql_client::Client;

use crate::{database, utils};
use ethers::{
    contract::Event,
    providers::{Http, Provider},
    types::{Address, BlockNumber},
    utils::Geth,
    Middleware,
};
use std::convert::TryFrom;
use std::sync::Arc;
use ethers::providers::Middleware;
use ethers::types::{Filter, H160, H256, U256};

/// Fetches data from GQL and insert it into DB
/// Long runtime for chains with large vote sets
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

/// Fetches historical votes from a certain block, optionally to a certain block, or to latest if not provided
/// Uses an RPC for speed instead of TheGraph. If fetching events from close to the tip of the chain, there can be reorgs and certain votes can be invalidated.
/// A function to remove invalidated votes will be provided.
pub async fn fetch_votes_for_round(pg: &mut PgConnection, chain_id: u16, round_id: Address, block_from: u16, block_to: Option<u16>) -> Result<(), Err()> {
    let provider = Provider::<Http>::try_from(HTTP_URL)?;
    let client = Arc::new(provider);
    let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();
    println!("last_block: {last_block}");

    let address = match chain_id {

    }

    // https://github.com/allo-protocol/contracts/blob/251d67ffe0519b153e78c5d6ff74bdd12ceef03f/contracts/votingStrategy/QuadraticFundingStrategy/QuadraticFundingVotingStrategyImplementation.sol#L27
    let filter = Filter::new()
        .address()
        .event("PoolCreated(address,address,uint24,int24,address)")
        .topic1(token_topics.to_vec())
        .topic2(token_topics.to_vec())
        .from_block(0);
    let logs = client.get_logs(&filter).await?;
    println!("{} pools found!", logs.iter().len());
    for log in logs.iter() {
        let token0 = Address::from(log.topics[1]);
        let token1 = Address::from(log.topics[2]);
        let fee_tier = U256::from_big_endian(&log.topics[3].as_bytes()[29..32]);
        let tick_spacing = U256::from_big_endian(&log.data[29..32]);
        let pool = Address::from(&log.data[44..64].try_into()?);
        println!(
            "pool = {pool}, token0 = {token0}, token1 = {token1}, fee = {fee_tier}, spacing = {tick_spacing}"
        );
    }

    Ok(())
}