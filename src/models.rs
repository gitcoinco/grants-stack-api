#![allow(non_snake_case)]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{programs, projects, projects_meta_ptrs, rounds, votes, voting_strategies, round_meta_ptrs, round_projects_meta_ptrs, project_meta_ptrs, qf_votes};

// PROGRAMS
#[derive(Clone, Insertable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = programs)]
pub struct Program {
    pub id: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub chainId: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ProgramsQuery {
    pub programs: Vec<Program>,
}

// PROJECTS
#[derive(Clone, Insertable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = projects)]
pub struct Project {
    pub id: String,
    pub payoutAddress: Option<String>,
    pub project: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub chainId: Option<String>,
    pub roundId: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ProjectsQuery {
    pub roundProjects: Vec<Project>,
}

// ROUNDS
#[derive(Clone, Insertable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = rounds)]
pub struct Round {
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

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectsMetaPtrThing {
    pub protocol: u16,
    pub pointer: String,
    pub id: String,
}
#[derive(Clone, Insertable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = projects_meta_ptrs)]
pub struct ProjectsMetaPtr {
    pub protocol: Option<String>,
    pub pointer: Option<String>,
    pub createdAt: Option<String>,
    pub updatedAt: Option<String>,
    pub roundId: String,
    pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RoundThing {
    pub id: String,
    pub payoutStrategy: String,
    pub token: String,
    pub roundStartTime: String,
    pub roundEndTime: String,
    pub applicationsStartTime: String,
    pub applicationsEndTime: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub projectsMetaPtr: Option<ProjectsMetaPtrThing>,
    pub chainId: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RoundsQuery {
    pub rounds: Vec<RoundThing>,
}

// VOTES
#[derive(Clone, Insertable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = votes)]
pub struct Vote {
    pub id: String,
    pub createdAt: String,
    pub amount: String,
    pub from: String,
    pub to: String,
    pub token: String,
    pub version: String,
    pub projectId: Option<String>,
    pub chainId: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct VotesQuery {
    pub qfvotes: Vec<Vote>,
}

/////////////////////
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
    pub chainId: String,
}

#[derive(Clone, Insertable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = projects)]
pub struct ProjectItem {
    pub id: String,
    pub payoutAddress: Option<String>,
    pub project: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub chainId: Option<String>,
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

