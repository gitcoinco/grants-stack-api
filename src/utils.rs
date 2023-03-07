use async_recursion::async_recursion;
use diesel::{Connection, ConnectionResult, PgConnection};
use gql_client::Client;
use serde::Deserialize;

use crate::models::{
    Program, ProgramsQuery, Project, ProjectsQuery, Round,
    RoundsQuery, Vote, VotesQuery, RoundThing,
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

#[derive(Deserialize, Debug, Clone)]
pub struct MetaPtr {
    pub id: String,
    pub pointer: String,
    pub protocol: u16,
}

#[derive(Deserialize, Debug, Clone)]
struct RoundsDerivedQuery<T> {
    pub rounds: Vec<T>,
}
// queries round round meta ptr
#[derive(Deserialize, Debug, Clone)]
pub struct RoundMetaPtr {
    pub id: String,
    pub roundMetaPtr: MetaPtr,
    pub chainId: Option<u16>,
}
/// Queries for the subgraph
#[async_recursion]
pub async fn r_query_round_meta_ptr(gql: &Client, last_id: &str, chain_id: u16) -> Vec<RoundMetaPtr> {
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

#[derive(Deserialize, Debug, Clone)]
pub struct VotingStrategyDerivedQuery<T> {
    votingStrategies: Vec<T>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct DerivedRoundId {
    pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct VotingStrategy {
    pub id: String,
    pub strategyAddress: String,
    pub strategyName: String,
    pub version: String,
    pub round: Option<DerivedRoundId>,
    pub chainId: Option<u16>,
}

// query: 
// {
//     votingStrategies {
//       id
//       strategyAddress
//       strategyName
//       version
//       round {
//         id
//       }
//     }
//   }

#[async_recursion]
pub async fn r_query_voting_strategies(gql: &Client, last_id: &str, chain_id: u16) -> Vec<VotingStrategy> {
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

#[derive(Deserialize, Debug, Clone)]
pub struct RoundProjectsDerivedQuery<T> {
    roundProjects: Vec<T>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectDerivedRoundId {
    pub id: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Project2 {
    pub id: String,
    pub project: String,
    pub payoutAddress: Option<String>,
    pub createdAt: String,
    pub updatedAt: String,
    pub round: ProjectDerivedRoundId,
    pub chainId: Option<u16>,
}

// query: 
// {
//     roundProjects {
//       id
//       createdAt
//       payoutAddress
//       project
//       updatedAt
//     }
//   }
#[async_recursion]
pub async fn r_query_round_projects(gql: &Client, last_id: &str, chain_id: u16) -> Vec<Project2> {
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
        .query::<RoundProjectsDerivedQuery<Project2>>(&query)
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


// query: 
// {
//     roundProjects {
//       id
//       round {
//         id
//       }
//       metaPtr {
//         id
//         pointer
//         protocol
//       }
//     }
//   }

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectMetaPtr {
    pub id: String,
    pub metaPtr: MetaPtr,
    pub round: ProjectDerivedRoundId,
    pub chainId: Option<u16>,
}

#[async_recursion]
pub async fn r_query_project_meta_ptrs(gql: &Client, last_id: &str, chain_id: u16) -> Vec<ProjectMetaPtr> {
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

// query : 
// {
//     qfvotes {
//       version
//       token
//       to
//       id
//       projectId
//       from
//       createdAt
//       amount
//       votingStrategy {
//         round {
//           id
//         }
//       }
//     }
//   }

#[derive(Deserialize, Debug, Clone)]
pub struct QfVotesDerivedQuery<T> {
    qfvotes: Vec<T>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct QfVote {
    pub id: String,
    pub version: String,
    pub token: String,
    pub from: String,
    pub to: String,
    pub amount: String,
    pub projectId: String,
    pub createdAt: String,
    pub votingStrategy: QfVotesDerivedVotingStrategy,
    pub chainId: Option<u16>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct QfVotesDerivedVotingStrategy {
    pub round: Option<QfVotesDerivedRoundId>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct QfVotesDerivedRoundId {
    pub id: String,
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

// query: 
// {
//     rounds {
//       id
//       payoutStrategy
//       roundEndTime
//       roundStartTime
//       token
//       updatedAt
//       createdAt
//       applicationsStartTime
//       applicationsEndTime
//     }
//   }

#[derive(Deserialize, Debug, Clone)]
pub struct Round1 {
    pub id: String,
    pub payoutStrategy: String,
    pub roundEndTime: String,
    pub roundStartTime: String,
    pub token: String,
    pub updatedAt: String,
    pub createdAt: String,
    pub applicationsStartTime: String,
    pub applicationsEndTime: String,
    pub chainId: Option<u16>,
}

#[async_recursion]
pub async fn r_query_rounds(gql: &Client, last_id: &str, chain_id: u16) -> Vec<Round1> {
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
        .query::<RoundsDerivedQuery<Round1>>(&query)
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

// query: 
// {
//     roundProjects {
//       id
//       createdAt
//       payoutAddress
//       project
//       status
//       updatedAt
//     }
//   }

#[derive(Deserialize, Debug, Clone)]
pub struct Projects1 {
    id: String,
    createdAt: String,
    payoutAddress: Option<String>,
    project: String,
    updatedAt: String,
    chainId: Option<u16>,
    round: Option<RoundProjectsDerivedRoundId>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RoundProjectsDerivedRoundId {
    id: String,
}

#[async_recursion]
pub async fn r_query_round_projects1(gql: &Client, last_id: &str, chain_id: u16) -> Vec<Projects1> {
    let query = format!(
        "
        query GetRoundProjectsQuery {{
            roundProjects(first: 1000, where: {{ id_gt: \"{}\" }}) {{
                id
                createdAt
                payoutAddress
                project
                status
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
        .query::<RoundProjectsDerivedQuery<Projects1>>(&query)
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
    let mut next_rounds = Box::pin(r_query_round_projects1(gql, &last_id, chain_id)).await;
    round_projects.append(&mut next_rounds);
    round_projects

}

// query: 
// {
//     rounds {
//       projectsMetaPtr {
//         id
//         pointer
//         protocol
//       }
//       id
//     }
//   }

#[derive(Deserialize, Debug, Clone)]
pub struct RoundProjectsMetaPtr {
    pub id: String,
    pub projectsMetaPtr: MetaPtr,
    pub chainId: Option<u16>,
}

#[async_recursion]
pub async fn r_query_round_projects_meta_ptrs1(gql: &Client, last_id: &str, chain_id: u16) -> Vec<RoundProjectsMetaPtr> {
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
    let mut next_rounds = Box::pin(r_query_round_projects_meta_ptrs1(gql, &last_id, chain_id)).await;
    round_projects_meta_ptrs.append(&mut next_rounds);
    round_projects_meta_ptrs

}

// #[derive(Deserialize, Debug, Clone)]
// pub struct RoundProjectsMetaPtrDerived {
//     projectsMetaPtr: RoundProjectsMetaPtr,
// }

// #[async_recursion]
// pub async fn r_query_round_projects_meta_ptrs1(gql: &Client, last_id: &str, chain_id: u16) -> Vec<RoundProjectsMetaPtr> {
//     let query = format!(
//         "
//         query GetRoundProjectsMetaPtrQuery {{
//             rounds(first: 1000, where: {{ id_gt: \"{}\", projectsMetaPtr_not: null }}) {{
//                 projectsMetaPtr {{
//                     id
//                     pointer
//                     protocol
//                 }}
//                 id
//             }}
//         }}
//         ",
//         last_id
//     );
//     let res = gql
//         .query::<RoundsDerivedQuery<RoundProjectsMetaPtrDerived>>(&query)
//         .await
//         .unwrap()
//         .expect("Error getting rounds");

//     let mut round_projects_meta_ptr = res.rounds.into_iter().map(|x| x.projectsMetaPtr).collect::<Vec<RoundProjectsMetaPtr>>();
//     // add chain id
//     round_projects_meta_ptr.iter_mut().for_each(|item| {
//         item.chainId = Option::from(chain_id);
//     });
//     if round_projects_meta_ptr.len() < 1000 {
//         return round_projects_meta_ptr;
//     }
//     let last_id = round_projects_meta_ptr.last().unwrap().id.clone();
//     let mut next_rounds = Box::pin(r_query_round_projects_meta_ptrs1(gql, &last_id, chain_id)).await;
//     round_projects_meta_ptr.append(&mut next_rounds);
//     round_projects_meta_ptr

    


//     // let mut round_projects_meta_ptr = res.rounds.into_iter().map(|x| x.projectsMetaPtr).collect::<Vec<RoundProjectsMetaPtr>>();
//     // // add chain id
//     // round_projects_meta_ptr.iter_mut().for_each(|item| {
//     //     item.chainId = Option::from(chain_id);
//     // });
//     // if round_projects_meta_ptr.len() < 1000 {
//     //     return round_projects_meta_ptr;
//     // }
//     // let last_id = round_projects_meta_ptr.last().unwrap().id.clone();
//     // let mut next_rounds = Box::pin(r_query_round_projects_meta_ptrs1(gql, &last_id, chain_id)).await;
//     // round_projects_meta_ptr.append(&mut next_rounds);
//     // round_projects_meta_ptr

// }