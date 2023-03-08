use std::collections::HashMap;

use async_recursion::async_recursion;
use diesel::{Connection, ConnectionResult, PgConnection};
use ethers::types::U256;
use gql_client::Client;

use crate::{
    database,
    models::{
        self, Project, ProjectMetaPtr, ProjectSummary, QfVote, QfVotesDerivedQuery, Round,
        RoundMetaPtr, RoundProjectsDerivedQuery, RoundProjectsMetaPtr, RoundsDerivedQuery,
        TokenVote, VotingStrategy, VotingStrategyDerivedQuery,
    },
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

/// Queries for the subgraph
#[async_recursion]
pub async fn r_query_round_meta_ptr(
    gql: &Client,
    last_id: &str,
    chain_id: u16,
) -> Vec<RoundMetaPtr> {
    let query = format!(
        "
        query GetRoundsMetaPtrQuery {{
            rounds(first: 1000, where: {{ id_gt: \"{}\" }}) {{
                id
                roundMetaPtr {{
                    id
                    pointer
                    protocol
                }}
            }}
        }}
        ",
        last_id
    );
    let res = gql
        .query::<RoundsDerivedQuery<RoundMetaPtr>>(&query)
        .await
        .unwrap()
        .expect("Error getting rounds");

    let mut round_meta_ptrs = res.rounds;
    // add chain id
    round_meta_ptrs.iter_mut().for_each(|item| {
        item.chainId = Option::from(chain_id);
    });

    if round_meta_ptrs.len() < 1000 {
        return round_meta_ptrs;
    }
    let last_id = round_meta_ptrs.last().unwrap().id.clone();
    let mut next_rounds = Box::pin(r_query_round_meta_ptr(gql, &last_id, chain_id)).await;
    round_meta_ptrs.append(&mut next_rounds);
    round_meta_ptrs
}

#[async_recursion]
pub async fn r_query_voting_strategies(
    gql: &Client,
    last_id: &str,
    chain_id: u16,
) -> Vec<VotingStrategy> {
    let query = format!(
        "
        query GetVotingStrategiesQuery {{
            votingStrategies(first: 1000, where: {{ id_gt: \"{}\", }}) {{
                id
                strategyAddress
                strategyName
                version
                round {{
                    id
                }}
            }}
        }}
        ",
        last_id
    );
    let res = gql
        .query::<VotingStrategyDerivedQuery<VotingStrategy>>(&query)
        .await
        .unwrap()
        .expect("Error getting rounds");

    let mut voting_strategies = res.votingStrategies;
    // add chain id
    voting_strategies.iter_mut().for_each(|item| {
        item.chainId = Option::from(chain_id);
    });
    if voting_strategies.len() < 1000 {
        return voting_strategies;
    }
    let last_id = voting_strategies.last().unwrap().id.clone();
    let mut next_rounds = Box::pin(r_query_voting_strategies(gql, &last_id, chain_id)).await;
    voting_strategies.append(&mut next_rounds);
    voting_strategies
}

#[async_recursion]
pub async fn r_query_round_projects(gql: &Client, last_id: &str, chain_id: u16) -> Vec<Project> {
    let query = format!(
        "
        query GetRoundProjectsQuery {{
            roundProjects(first: 1000, where: {{ id_gt: \"{}\" }}) {{
                id
                createdAt
                payoutAddress
                project
                updatedAt
                round {{
                    id
                }}
            }}
        }}
        ",
        last_id
    );
    let res = gql
        .query::<RoundProjectsDerivedQuery<Project>>(&query)
        .await
        .unwrap()
        .expect("Error getting rounds");

    let mut round_projects = res.roundProjects;
    // add chain id
    round_projects.iter_mut().for_each(|item| {
        item.chainId = Option::from(chain_id);
    });
    if round_projects.len() < 1000 {
        return round_projects;
    }
    let last_id = round_projects.last().unwrap().id.clone();
    let mut next_rounds = Box::pin(r_query_round_projects(gql, &last_id, chain_id)).await;
    round_projects.append(&mut next_rounds);
    round_projects
}

#[async_recursion]
pub async fn r_query_project_meta_ptrs(
    gql: &Client,
    last_id: &str,
    chain_id: u16,
) -> Vec<ProjectMetaPtr> {
    let query = format!(
        "
        query GetProjectMetaPtrQuery {{
            roundProjects(first: 1000, where: {{ id_gt: \"{}\", metaPtr_not: null}}) {{
                id
                metaPtr {{
                    id
                    pointer
                    protocol
                }}
                round {{
                    id
                }}
            }}
        }}
        ",
        last_id
    );
    let res = gql
        .query::<RoundProjectsDerivedQuery<ProjectMetaPtr>>(&query)
        .await
        .unwrap()
        .expect("Error getting rounds");

    let mut round_projects_meta_ptrs = res.roundProjects;
    // add chain id
    round_projects_meta_ptrs.iter_mut().for_each(|item| {
        item.chainId = Option::from(chain_id);
    });
    if round_projects_meta_ptrs.len() < 1000 {
        return round_projects_meta_ptrs;
    }
    let last_id = round_projects_meta_ptrs.last().unwrap().id.clone();
    let mut next_rounds = Box::pin(r_query_project_meta_ptrs(gql, &last_id, chain_id)).await;
    round_projects_meta_ptrs.append(&mut next_rounds);
    round_projects_meta_ptrs
}

#[async_recursion]
pub async fn r_query_qf_votes(gql: &Client, last_id: &str, chain_id: u16) -> Vec<QfVote> {
    let query = format!(
        "
        query GetQfVotesQuery {{
            qfvotes(first: 1000, where: {{ id_gt: \"{}\" }}) {{
                id
                version
                token
                from
                to
                amount
                projectId
                createdAt
                votingStrategy {{
                    round {{
                        id
                    }}
                }}
            }}
        }}
        ",
        last_id
    );
    let res = gql
        .query::<QfVotesDerivedQuery<QfVote>>(&query)
        .await
        .unwrap()
        .expect("Error getting rounds");

    let mut qf_votes = res.qfvotes;
    // add chain id
    qf_votes.iter_mut().for_each(|item| {
        item.chainId = Option::from(chain_id);
    });
    if qf_votes.len() < 1000 {
        return qf_votes;
    }
    let last_id = qf_votes.last().unwrap().id.clone();
    let mut next_rounds = Box::pin(r_query_qf_votes(gql, &last_id, chain_id)).await;
    qf_votes.append(&mut next_rounds);
    qf_votes
}

// Fetch data from the IPFS Gateway of choice
pub async fn fetch_from_ipfs(cid: &str) -> Result<serde_json::Value, reqwest::Error> {
    let gateway: String = dotenv::var("PINATA_GATEWAY").expect("no gateway");
    let url = format!("https://{}/ipfs/{}", gateway, cid);
    let response = reqwest::get(&url).await.expect("Error getting IPFS data");
    response.json::<serde_json::Value>().await
}

#[async_recursion]
pub async fn r_query_rounds(gql: &Client, last_id: &str, chain_id: u16) -> Vec<Round> {
    let query = format!(
        "
        query GetRoundsQuery {{
            rounds(first: 1000, where: {{ id_gt: \"{}\" }}) {{
                id
                payoutStrategy
                roundEndTime
                roundStartTime
                token
                updatedAt
                createdAt
                applicationsStartTime
                applicationsEndTime
            }}
        }}
        ",
        last_id
    );
    let res = gql
        .query::<RoundsDerivedQuery<Round>>(&query)
        .await
        .unwrap()
        .expect("Error getting rounds");

    let mut rounds = res.rounds;
    // add chain id
    rounds.iter_mut().for_each(|item| {
        item.chainId = Option::from(chain_id);
    });
    if rounds.len() < 1000 {
        return rounds;
    }
    let last_id = rounds.last().unwrap().id.clone();
    let mut next_rounds = Box::pin(r_query_rounds(gql, &last_id, chain_id)).await;
    rounds.append(&mut next_rounds);
    rounds
}

#[async_recursion]
pub async fn r_query_round_projects_meta_ptrs(
    gql: &Client,
    last_id: &str,
    chain_id: u16,
) -> Vec<RoundProjectsMetaPtr> {
    let query = format!(
        "
        query GetRoundProjectsMetaPtrQuery {{
            rounds(first: 1000, where: {{ id_gt: \"{}\", projectsMetaPtr_not: null }}) {{
                projectsMetaPtr {{
                    id
                    pointer
                    protocol
                }}
                id
            }}
        }}
        ",
        last_id
    );
    let res = gql
        .query::<RoundsDerivedQuery<RoundProjectsMetaPtr>>(&query)
        .await
        .unwrap()
        .expect("Error getting rounds");

    let mut round_projects_meta_ptrs = res.rounds;
    // add chain id
    round_projects_meta_ptrs.iter_mut().for_each(|item| {
        item.chainId = Option::from(chain_id);
    });
    if round_projects_meta_ptrs.len() < 1000 {
        return round_projects_meta_ptrs;
    }
    let last_id = round_projects_meta_ptrs.last().unwrap().id.clone();
    let mut next_rounds = Box::pin(r_query_round_projects_meta_ptrs(gql, &last_id, chain_id)).await;
    round_projects_meta_ptrs.append(&mut next_rounds);
    round_projects_meta_ptrs
}

pub async fn summarize_project(
    conn: &mut PgConnection,
    project_id: String,
) -> models::ProjectSummary {
    // get project data and votes from db
    let project_data = database::get_project_data(conn, project_id.clone()).await;
    let project_votes = database::get_project_votes(conn, project_id.clone()).await;

    let project_id = project_data[0].projectId.clone();
    let round_id = project_data[0].roundId.clone();

    // count the number of votes for the project
    let vote_count = project_votes.len() as i64;

    // coerce vote token and amount to TokenVote type
    // (converting the amount to U256 for math operations)
    let mut token_votes: Vec<TokenVote> = Vec::new();
    for vote in project_votes.clone() {
        let token_vote = TokenVote {
            token: vote.token,
            amount: U256::from_dec_str(vote.amount.as_str()).unwrap(),
        };
        token_votes.push(token_vote);
    }

    // sum the votes for unique token
    let mut token_vote_map: HashMap<String, U256> = HashMap::new();
    for vote in token_votes {
        let token = vote.token;
        let amount = vote.amount;
        let current_amount = token_vote_map.entry(token).or_insert(U256::from(0));
        *current_amount += amount;
    }

    // convert token_vote_map to TokenVote type
    let mut token_votes: Vec<TokenVote> = Vec::new();
    for (token, amount) in token_vote_map {
        let token_vote = TokenVote { token, amount };
        token_votes.push(token_vote);
    }

    // count the number of unique vote from address
    // we use a hashmap here for future use cases where we want to count the number of votes for a specific address
    let mut address_vote_map: HashMap<String, i64> = HashMap::new();
    for vote in project_votes {
        let address = vote.from;
        let current_count = address_vote_map.entry(address).or_insert(0);
        *current_count += 1;
    }
    let unique_voter_count = address_vote_map.len() as i64;

    let summary: ProjectSummary = ProjectSummary {
        project_id: project_id,
        round_id: round_id,
        vote_count: vote_count,
        unique_voter_count: unique_voter_count,
        vote_token_sum: token_votes,
    };
    summary
}
