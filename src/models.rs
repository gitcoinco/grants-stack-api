#![allow(non_snake_case)]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{programs, projects, projects_meta_ptrs, rounds, votes};

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
    pub status: String,
    pub payoutAddress: Option<String>,
    pub project: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub chainId: Option<String>,
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

#[derive(Deserialize, Debug)]
pub struct RoundsQuery {
    pub rounds: Vec<Round>,
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

// #[derive(Deserialize, Debug)]
#[derive(Clone, Insertable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = projects_meta_ptrs)]
pub struct ProjectsMetaPtr {
    pub pointer: String,
    pub protocol: String,
    pub roundId: String,
}
#[derive(Deserialize, Debug)]
pub struct ProjectsMetaPtrQuery {
    pub id: String, // round ID
    pub projectsMetaPtr: Option<ProjectsMetaPtr>,
}
#[derive(Deserialize, Debug)]
pub struct MetaPtrQuery {
    pub rounds: Vec<ProjectsMetaPtrQuery>,
}
