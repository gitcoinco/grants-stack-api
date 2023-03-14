#![allow(non_snake_case)]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    project_meta_ptrs, projects, qf_votes, round_meta_ptrs, round_projects_meta_ptrs, rounds,
    token_prices, voting_strategies,
};
use ethers::types::U256;

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
    pub payoutStrategy: Option<String>,
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
    pub payoutStrategy: Option<String>,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct TokenVote {
    pub amount: U256,
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectSummary {
    pub project_id: String,
    pub round_id: String,
    pub vote_count: i64,
    pub unique_voter_count: i64,
    pub vote_token_sum: Vec<TokenVote>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RoundSummary {
    pub round_id: String,
    pub vote_count: i64,
    pub unique_voter_count: i64,
    pub vote_token_sum: Vec<TokenVote>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectResponseData {
    pub data: Option<ProjectItem>,
    pub project_meta_ptr: Option<ProjectMetaPtrItem>,
    pub project_votes: Option<Vec<QfVoteItem>>,
    pub project_summary: Option<ProjectSummary>,
}

#[derive(Clone, Deserialize)]
pub struct GetProjectDataQueryParams {
    pub project_id: String,
    pub data: Option<bool>,
    pub project_meta_ptr: Option<bool>,
    pub project_votes: Option<bool>,
    pub project_summary: Option<bool>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct GetRoundDataQueryParams {
    pub round_id: String,
    pub data: Option<bool>,
    pub round_meta_ptr: Option<bool>,
    pub voting_strategy: Option<bool>,
    pub projects_meta_ptr: Option<bool>,
    pub round_projects: Option<bool>,
    pub round_votes: Option<bool>,
    pub round_summary: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RoundResponseData {
    pub data: Option<RoundItem>,
    pub round_meta_ptr: Option<RoundMetaPtrItem>,
    pub voting_strategy: Option<VotingStrategyItem>,
    pub projects_meta_ptr: Option<RoundProjectsMetaPtrItem>,
    pub round_projects: Option<Vec<ProjectItem>>,
    pub round_votes: Option<Vec<QfVoteItem>>,
    pub round_summary: Option<RoundSummary>,
}

#[derive(Deserialize)]
pub struct GetIPFSQueryParams {
    pub cid: Option<String>,
}

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = token_prices)]
pub struct TokenPriceItem {
    pub timestamp: String,
    pub token: String,
    pub price: String,
    pub chainId: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TokenPrice {
    pub timestamp: u64,
    pub usd: f64,
}

#[derive(Deserialize, Debug)]
pub struct TokenPriceResponse {
    pub prices: Vec<TokenPrice>,
}
