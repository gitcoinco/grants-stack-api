use diesel::{PgConnection, QueryDsl, RunQueryDsl};

use crate::models::{Program, Project, ProjectsMetaPtr, Round, Vote};
use crate::schema::programs::dsl::*;
use crate::schema::projects::dsl::*;
use crate::schema::rounds::dsl::*;
use crate::schema::votes::dsl::*;
use crate::schema::{programs, projects, projects_meta_ptrs, rounds, votes};
use diesel::ExpressionMethods;

pub fn new_program(conn: &mut PgConnection, data: Program) {
    let programs_data = vec![data];
    insert_programs(conn, programs_data);
}

pub fn new_round(conn: &mut PgConnection, data: Round) {
    let rounds_data = vec![data];
    insert_rounds(conn, rounds_data);
}

pub fn new_project(conn: &mut PgConnection, data: Project) {
    let projects_data = vec![data];
    insert_projects(conn, projects_data);
}

pub fn new_vote(conn: &mut PgConnection, data: Vote) {
    let votes_data = vec![data];
    insert_votes(conn, votes_data);
}

pub fn new_projects(conn: &mut PgConnection, data: Vec<Project>) {
    let chunk_size = 1000;
    let mut projects_data = data;

    while projects_data.len() > chunk_size {
        let (chunk, rest) = projects_data.split_at(chunk_size);
        insert_projects(conn, chunk.to_vec());
        projects_data = rest.to_vec();
    }

    insert_projects(conn, projects_data);
}

pub fn new_rounds(conn: &mut PgConnection, data: Vec<Round>) {
    let chunk_size = 1000;
    let mut rounds_data = data;

    while rounds_data.len() > chunk_size {
        let (chunk, rest) = rounds_data.split_at(chunk_size);
        insert_rounds(conn, chunk.to_vec());
        rounds_data = rest.to_vec();
    }

    insert_rounds(conn, rounds_data);
}

pub fn new_programs(conn: &mut PgConnection, data: Vec<Program>) {
    let chunk_size = 1000;
    let mut programs_data = data;

    while programs_data.len() > chunk_size {
        let (chunk, rest) = programs_data.split_at(chunk_size);
        insert_programs(conn, chunk.to_vec());
        programs_data = rest.to_vec()
    }

    insert_programs(conn, programs_data);
}

pub fn new_votes(conn: &mut PgConnection, data: Vec<Vote>) {
    let chunk_size = 1000;
    let mut votes_data = data;

    while votes_data.len() > chunk_size {
        let (chunk, rest) = votes_data.split_at(chunk_size);
        insert_votes(conn, chunk.to_vec());
        votes_data = rest.to_vec()
    }

    insert_votes(conn, votes_data);
}

pub fn new_projects_meta_ptrs(conn: &mut PgConnection, data: Vec<ProjectsMetaPtr>) {
    let chunk_size = 1000;
    let mut projects_meta_ptrs_data = data;

    while projects_meta_ptrs_data.len() > chunk_size {
        let (chunk, rest) = projects_meta_ptrs_data.split_at(chunk_size);
        insert_project_meta_ptrs(conn, chunk.to_vec());
        projects_meta_ptrs_data = rest.to_vec()
    }

    insert_project_meta_ptrs(conn, projects_meta_ptrs_data);
}

fn insert_rounds(conn: &mut PgConnection, data: Vec<Round>) {
    // insert into round table, ignore duplicates
    diesel::insert_into(rounds::table)
        .values(&data)
        .on_conflict(rounds::id)
        .do_nothing()
        .execute(conn)
        .expect("Error saving new round");
}

fn insert_programs(conn: &mut PgConnection, data: Vec<Program>) {
    // insert into program table, ignore duplicates
    diesel::insert_into(programs::table)
        .values(&data)
        .on_conflict(programs::id)
        .do_nothing()
        .execute(conn)
        .expect("Error saving new program");
}

fn insert_votes(conn: &mut PgConnection, data: Vec<Vote>) {
    // insert into vote table, ignore duplicates
    diesel::insert_into(votes::table)
        .values(&data)
        .on_conflict(votes::id)
        .do_nothing()
        .execute(conn)
        .expect("Error saving new vote");
}

fn insert_projects(conn: &mut PgConnection, data: Vec<Project>) {
    // insert into project table, ignore duplicates
    diesel::insert_into(projects::table)
        .values(&data)
        .on_conflict(projects::id)
        .do_nothing()
        .execute(conn)
        .expect("Error saving new project");
}

fn insert_project_meta_ptrs(conn: &mut PgConnection, data: Vec<ProjectsMetaPtr>) {
    // insert into project_meta_ptr table, ignore duplicates
    diesel::insert_into(projects_meta_ptrs::table)
        .values(&data)
        .on_conflict(projects_meta_ptrs::roundId)
        .do_nothing()
        .execute(conn)
        .expect("Error saving new project_meta_ptr");
}

pub async fn get_programs(conn: &mut PgConnection) -> Vec<Program> {
    programs
        .load::<Program>(conn)
        .expect("Error loading programs")
}

pub async fn get_rounds(conn: &mut PgConnection) -> Vec<Round> {
    rounds.load::<Round>(conn).expect("Error loading rounds")
}

pub async fn get_projects(conn: &mut PgConnection) -> Vec<Project> {
    projects
        .load::<Project>(conn)
        .expect("Error loading projects")
}

pub async fn get_votes(conn: &mut PgConnection) -> Vec<Vote> {
    votes.load::<Vote>(conn).expect("Error loading votes")
}

pub async fn get_votes_of_project_id(conn: &mut PgConnection, project_id: &str) -> Vec<Vote> {
    votes
        .filter(projectId.eq(project_id))
        .load::<Vote>(conn)
        .expect("Error loading votes")
}
