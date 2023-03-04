use async_recursion::async_recursion;
use diesel::{Connection, ConnectionResult, PgConnection};
use gql_client::Client;

use crate::models::{
    Program, ProgramsQuery, Project, ProjectsQuery, Round, RoundsQuery, Vote, VotesQuery,
};

pub const ETHEREUM_MAINNET: u16 = 1;
pub const ETHEREUM_GOERLI: u16 = 5;
pub const OPTIMISM_MAINNET: u16 = 10;
pub const FANTOM_MAINNET: u16 = 250;
pub const FANTOM_TESTNET: u16 = 4002;

// Establish the connection with the Postgres database
pub fn establish_pg_connection() -> ConnectionResult<PgConnection> {
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
}

// Check if the chain id is valid
pub fn check_chain_id(chain_id: u16) -> bool {
    chain_id == ETHEREUM_MAINNET
        || chain_id == ETHEREUM_GOERLI
        || chain_id == OPTIMISM_MAINNET
        || chain_id == FANTOM_MAINNET
        || chain_id == FANTOM_TESTNET
}

// Establish the GraphQL client for the chain_id
pub fn establish_gql_client(chain_id: u16) -> Client {
    let gql_url: String = match chain_id {
        ETHEREUM_MAINNET => dotenv::var("SUBGRAPH_ETHEREUM_MAINNET_API")
            .expect("SUBGRAPH_ETHEREUM_MAINNET_API must be set"),
        ETHEREUM_GOERLI => dotenv::var("SUBGRAPH_ETHEREUM_GOERLI_API")
            .expect("SUBGRAPH_ETHEREUM_GOERLI_API must be set"),
        OPTIMISM_MAINNET => dotenv::var("SUBGRAPH_OPTIMISM_MAINNET_API")
            .expect("SUBGRAPH_OPTIMISM_MAINNET_API must be set"),
        FANTOM_MAINNET => dotenv::var("SUBGRAPH_FANTOM_MAINNET_API")
            .expect("SUBGRAPH_FANTOM_MAINNET_API must be set"),
        FANTOM_TESTNET => dotenv::var("SUBGRAPH_FANTOM_TESTNET_API")
            .expect("SUBGRAPH_FANTOM_TESTNET_API must be set"),
        _ => panic!("Chain ID not supported"),
    };

    Client::new(gql_url)
}

// add_*_chain_id is meant to retoractively add the chain ID of the data
// TODO: See about a generic funtion to avoid duplicated code
pub async fn add_program_chain_id(data: Vec<Program>, chain_id: u16) -> Vec<Program> {
    data.iter()
        .map(|item| {
            let mut item = item.clone();
            item.chainId = Option::from(chain_id.to_string());
            item
        })
        .collect()
}
pub async fn add_round_chain_id(data: Vec<Round>, chain_id: u16) -> Vec<Round> {
    data.iter()
        .map(|item| {
            let mut item = item.clone();
            item.chainId = Option::from(chain_id.to_string());
            item
        })
        .collect()
}
pub async fn add_project_chain_id(data: Vec<Project>, chain_id: u16) -> Vec<Project> {
    data.iter()
        .map(|item| {
            let mut item = item.clone();
            item.chainId = Option::from(chain_id.to_string());
            item
        })
        .collect()
}
pub async fn add_vote_chain_id(data: Vec<Vote>, chain_id: u16) -> Vec<Vote> {
    data.iter()
        .map(|item| {
            let mut item = item.clone();
            item.chainId = Option::from(chain_id.to_string());
            item
        })
        .collect()
}
// TOOD: Create a query_ functions such that the queries are configuable in a some way
// Query all programs
#[async_recursion]
pub async fn r_query_programs(gql: &Client, last_id: &str) -> Vec<Program> {
    let query = format!(
        "
        query GetProgramsQuery {{
            programs(first: 1000, where: {{ id_gt: \"{}\" }}) {{
                id
                createdAt
                updatedAt
            }}
        }}
        ",
        last_id
    );
    let res = gql
        .query::<ProgramsQuery>(&query)
        .await
        .unwrap()
        .expect("Error getting programs");
    let mut programs = res.programs;
    if programs.len() < 1000 {
        return programs;
    }
    let last_id = programs.last().unwrap().id.clone();
    let mut next_programs = Box::pin(r_query_programs(gql, &last_id)).await;
    programs.append(&mut next_programs);
    programs
}

// Queries all rounds
#[async_recursion]
pub async fn r_query_rounds(gql: &Client, last_id: &str) -> Vec<Round> {
    let query = format!(
        "
        query GetRoundsQuery {{
            rounds(first: 1000, where: {{ id_gt: \"{}\" }}) {{
                id
                payoutStrategy
                token
                roundStartTime
                roundEndTime
                applicationsStartTime
                applicationsEndTime
                createdAt
                updatedAt
            }}
        }}
        ",
        last_id
    );
    let res = gql
        .query::<RoundsQuery>(&query)
        .await
        .unwrap()
        .expect("Error getting rounds");

    let mut rounds = res.rounds;
    if rounds.len() < 1000 {
        return rounds;
    }
    let last_id = rounds.last().unwrap().id.clone();
    let mut next_rounds = Box::pin(r_query_rounds(gql, &last_id)).await;
    rounds.append(&mut next_rounds);
    rounds
}

// Query all projects
#[async_recursion]
pub async fn r_query_projects(gql: &Client, last_id: &str) -> Vec<Project> {
    let query = format!(
        "
        query GetProjectQuery {{
            roundProjects(first: 1000, where: {{ id_gt: \"{}\" }}) {{
                id
                status
                payoutAddress
                project
                createdAt
                updatedAt
            }}
        }}
        ",
        last_id
    );

    let res = gql
        .query::<ProjectsQuery>(&query)
        .await
        .unwrap()
        .expect("Error getting projects");
    let mut projects = res.roundProjects;
    if projects.len() < 1000 {
        return projects;
    }
    let last_id = projects.last().unwrap().id.clone();
    let mut next_projects = Box::pin(r_query_projects(gql, &last_id)).await;
    projects.append(&mut next_projects);
    projects
}

// Query all votes
#[async_recursion]
pub async fn r_query_votes(gql: &Client, last_id: &str) -> Vec<Vote> {
    let query = format!(
        "
        query GetVotesQuery {{
            qfvotes(first: 1000, where: {{ id_gt: \"{}\" }}) {{
                id
                createdAt
                amount
                from
                to
                version
                token
                projectId
            }}
        }}
        ",
        last_id
    );
    let res = gql
        .query::<VotesQuery>(&query)
        .await
        .unwrap()
        .expect("Error getting votes");

    let mut votes = res.qfvotes;
    if votes.len() < 1000 {
        return votes;
    }
    let last_id = votes.last().unwrap().id.clone();
    let mut next_votes = Box::pin(r_query_votes(gql, &last_id)).await;
    votes.append(&mut next_votes);
    votes
}

// Fetch data from the IPFS Gateway of choice
pub async fn fetch_from_ipfs(cid: &str) -> Result<serde_json::Value, reqwest::Error> {
    let gateway: String = dotenv::var("PINATA_GATEWAY").expect("no gateway");
    let url = format!("https://{}/ipfs/{}", gateway, cid);
    let response = reqwest::get(&url).await.expect("Error getting IPFS data");
    response.json::<serde_json::Value>().await
}

// Backfill of the project id for version <0.2.0 vote
pub async fn backfill_project_id(votes: Vec<Vote>) -> Vec<Vote> {
    let mut votes = votes;
    let mut votes_to_update = Vec::new();
    for vote in votes.iter_mut() {
        if vote.version == "0.1.0" {
            println!("Backfilling project id for vote {}", vote.id);
        }
    }
    votes_to_update
}

pub struct MetaPtr {
    protocol: u16,
    pointer: String,
}
// TODO: Support other protocls
pub async fn fetch_metaptr_data(meta: MetaPtr) -> Result<serde_json::Value, reqwest::Error> {
    fetch_from_ipfs(&meta.pointer).await
}
