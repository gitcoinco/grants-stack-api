#![allow(non_snake_case)]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    project_meta_ptrs, projects, qf_votes, round_meta_ptrs, round_projects_meta_ptrs, rounds,
    voting_strategies,
};

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

#[derive(Deserialize, Debug, Clone)]
pub struct RoundsDerivedQuery<T> {
    pub rounds: Vec<T>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct VotingStrategyDerivedQuery<T> {
    pub votingStrategies: Vec<T>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RoundProjectsDerivedQuery<T> {
    pub roundProjects: Vec<T>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct QfVotesDerivedQuery<T> {
    pub qfvotes: Vec<T>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct DerivedRoundId {
    pub id: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct QfVotesDerivedVotingStrategy {
    pub round: Option<DerivedRoundId>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Round {
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
#[derive(Deserialize, Debug, Clone)]
pub struct Project {
    pub id: String,
    pub project: String,
    pub payoutAddress: Option<String>,
    pub createdAt: String,
    pub updatedAt: String,
    pub round: DerivedRoundId,
    pub chainId: Option<u16>,
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
pub struct VotingStrategy {
    pub id: String,
    pub strategyAddress: String,
    pub strategyName: String,
    pub version: String,
    pub round: Option<DerivedRoundId>,
    pub chainId: Option<u16>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MetaPtr {
    pub id: String,
    pub pointer: String,
    pub protocol: u16,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RoundMetaPtr {
    pub id: String,
    pub roundMetaPtr: MetaPtr,
    pub chainId: Option<u16>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ProjectMetaPtr {
    pub id: String,
    pub metaPtr: MetaPtr,
    pub round: DerivedRoundId,
    pub chainId: Option<u16>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RoundProjectsMetaPtr {
    pub id: String,
    pub projectsMetaPtr: MetaPtr,
    pub chainId: Option<u16>,
}
