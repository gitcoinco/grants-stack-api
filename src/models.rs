#![allow(non_snake_case)]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{projects, rounds, voting_strategies, round_meta_ptrs, round_projects_meta_ptrs, project_meta_ptrs, qf_votes};


#[derive(Clone, Insertable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = voting_strategies)]
pub struct VotingStrategyItem {
    pub id: String,
    pub strategyAddress: String,
    pub strategyName: String,
    pub version: String,
    pub roundId: String,
    pub chainId: String,
}

#[derive(Clone, Insertable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = round_meta_ptrs)]
pub struct RoundMetaPtrItem {
    pub id: String,
    pub protocol: String,
    pub pointer: String,
    pub roundId: String,
    pub chainId: String,
}


#[derive(Clone, Insertable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = round_projects_meta_ptrs)]
pub struct RoundProjectsMetaPtrItem {
    pub id: String,
    pub protocol: String,
    pub pointer: String,
    pub roundId: String,
    pub chainId: String,
}



#[derive(Clone, Insertable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = rounds)]
pub struct RoundItem {
    pub id: String,
    pub payoutStrategy: String,
    pub token: String,
    pub roundStartTime: String,
    pub roundEndTime: String,
    pub applicationsStartTime: String,
    pub applicationsEndTime: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub chainId: Option<String>,
}

#[derive(Clone, Insertable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = project_meta_ptrs)]
pub struct ProjectMetaPtrItem {
    pub id: String,
    pub protocol: String,
    pub pointer: String,
    pub roundId: String,
    pub projectId: String,
    pub chainId: String,
}

#[derive(Clone, Insertable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = projects)]
pub struct ProjectItem {
    pub id: String,
    pub payoutAddress: Option<String>,
    pub projectId: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub chainId: String,
    pub roundId: String,
}


#[derive(Clone, Insertable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = qf_votes)]
pub struct QfVoteItem {
    pub id: String,
    pub createdAt: String,
    pub amount: String,
    pub from: String,
    pub to: String,
    pub projectId: String,
    pub token: String,
    pub version: String,
    pub chainId: String,
    pub roundId: String,
}

