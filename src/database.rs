use actix_web::Error;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

use crate::models::{Program, Project, Round, Vote, ProjectsMetaPtr, VotingStrategyItem, RoundMetaPtrItem, RoundProjectsMetaPtrItem, RoundItem, ProjectMetaPtrItem, ProjectItem, QfVoteItem};
use crate::schema::programs::dsl::*;
use crate::schema::projects::dsl::*;
use crate::schema::rounds::dsl::*;
use crate::schema::votes::dsl::*;
use crate::schema::round_meta_ptrs::dsl::*; 
use crate::schema::{programs, projects, projects_meta_ptrs, rounds, votes, voting_strategies, round_meta_ptrs, round_projects_meta_ptrs, project_meta_ptrs, qf_votes};
use crate::utils::{VotingStrategy, self, RoundMetaPtr, RoundProjectsMetaPtr, Round1, ProjectMetaPtr, Project2, QfVote};
use diesel::ExpressionMethods;

// pub fn new_program(conn: &mut PgConnection, data: Program) {
//     let programs_data = vec![data];
//     insert_programs(conn, programs_data);
// }

// pub fn new_round(conn: &mut PgConnection, data: Round) {
//     let rounds_data = vec![data];
//     insert_rounds(conn, rounds_data);
// }

// pub fn new_project(conn: &mut PgConnection, data: Project) {
//     let projects_data = vec![data];
//     insert_projects(conn, projects_data);
// }

// pub fn new_vote(conn: &mut PgConnection, data: Vote) {
//     let votes_data = vec![data];
//     insert_votes(conn, votes_data);
// }

// pub fn new_projects(conn: &mut PgConnection, data: Vec<Project>) {
//     let chunk_size = 1000;
//     let mut projects_data = data;

//     while projects_data.len() > chunk_size {
//         let (chunk, rest) = projects_data.split_at(chunk_size);
//         insert_projects(conn, chunk.to_vec());
//         projects_data = rest.to_vec();
//     }

//     insert_projects(conn, projects_data);
// }

// pub fn new_rounds(conn: &mut PgConnection, data: Vec<Round>) {
//     let chunk_size = 1000;
//     let mut rounds_data = data;

//     while rounds_data.len() > chunk_size {
//         let (chunk, rest) = rounds_data.split_at(chunk_size);
//         insert_rounds(conn, chunk.to_vec());
//         rounds_data = rest.to_vec();
//     }

//     insert_rounds(conn, rounds_data);
// }

// pub fn new_programs(conn: &mut PgConnection, data: Vec<Program>) {
//     let chunk_size = 1000;
//     let mut programs_data = data;

//     while programs_data.len() > chunk_size {
//         let (chunk, rest) = programs_data.split_at(chunk_size);
//         insert_programs(conn, chunk.to_vec());
//         programs_data = rest.to_vec()
//     }

//     insert_programs(conn, programs_data);
// }

// pub fn new_votes(conn: &mut PgConnection, data: Vec<Vote>) {
//     let chunk_size = 1000;
//     let mut votes_data = data;

//     while votes_data.len() > chunk_size {
//         let (chunk, rest) = votes_data.split_at(chunk_size);
//         insert_votes(conn, chunk.to_vec());
//         votes_data = rest.to_vec()
//     }

//     insert_votes(conn, votes_data);
// }

// pub fn new_projects_meta_ptrs(conn: &mut PgConnection, data: Vec<ProjectsMetaPtr>) {
//     let chunk_size = 1000;
//     let mut projects_meta_ptrs_data = data;

//     while projects_meta_ptrs_data.len() > chunk_size {
//         let (chunk, rest) = projects_meta_ptrs_data.split_at(chunk_size);
//         insert_project_meta_ptrs(conn, chunk.to_vec());
//         projects_meta_ptrs_data = rest.to_vec()
//     }

//     insert_project_meta_ptrs(conn, projects_meta_ptrs_data);
// }

// fn insert_rounds(conn: &mut PgConnection, data: Vec<Round>) {
//     // insert into round table, ignore duplicates
//     diesel::insert_into(rounds::table)
//         .values(&data)
//         .on_conflict(rounds::id)
//         .do_nothing()
//         .execute(conn)
//         .expect("Error saving new round");
// }

// fn insert_programs(conn: &mut PgConnection, data: Vec<Program>) {
//     // insert into program table, ignore duplicates
//     diesel::insert_into(programs::table)
//         .values(&data)
//         .on_conflict(programs::id)
//         .do_nothing()
//         .execute(conn)
//         .expect("Error saving new program");
// }

// fn insert_votes(conn: &mut PgConnection, data: Vec<Vote>) {
//     // insert into vote table, ignore duplicates
//     diesel::insert_into(votes::table)
//         .values(&data)
//         .on_conflict(votes::id)
//         .do_nothing()
//         .execute(conn)
//         .expect("Error saving new vote");
// }

// fn insert_projects(conn: &mut PgConnection, data: Vec<Project>) {
//     // insert into project table, ignore duplicates
//     diesel::insert_into(projects::table)
//         .values(&data)
//         .on_conflict(projects::id)
//         .do_nothing()
//         .execute(conn)
//         .expect("Error saving new project");
// }

// fn insert_project_meta_ptrs(conn: &mut PgConnection, data: Vec<ProjectsMetaPtr>) {
//     // insert into project_meta_ptr table, ignore duplicates
//     diesel::insert_into(projects_meta_ptrs::table)
//         .values(&data)
//         .on_conflict(projects_meta_ptrs::id)
//         .do_nothing()
//         .execute(conn)
//         .expect("Error saving new project_meta_ptr");
// }

// pub async fn get_programs(conn: &mut PgConnection) -> Vec<Program> {
//     programs
//         .load::<Program>(conn)
//         .expect("Error loading programs")
// }

// pub async fn get_rounds(conn: &mut PgConnection) -> Vec<Round> {
//     rounds.load::<Round>(conn).expect("Error loading rounds")
// }

// pub async fn get_projects(conn: &mut PgConnection) -> Vec<Project> {
//     projects
//         .load::<Project>(conn)
//         .expect("Error loading projects")
// }

// pub async fn get_votes(conn: &mut PgConnection) -> Vec<Vote> {
//     votes.load::<Vote>(conn).expect("Error loading votes")
// }

// pub async fn get_votes_of_project_id(conn: &mut PgConnection, project_id: &str) -> Vec<Vote> {
//     votes
//         .filter(projectId.eq(project_id))
//         .load::<Vote>(conn)
//         .expect("Error loading votes")
// }

// pub async fn get_round_meta_ptr(conn: &mut PgConnection, round_id: &str) -> Vec<RoundMetaPtrItem> {
//     round_meta_ptrs
//         .filter(id.eq(round_id))
//         .load::<RoundMetaPtrItem>(conn)
//         .expect("Error loading round_meta_ptr")
// }

// pub async fn get_round_meta_ptr(conn: &mut PgConnection, round_id: &str) -> Vec<RoundMetaPtrItem> {
//     round_meta_ptrs
//         .filter(id.eq(round_id))
//         .load::<RoundMetaPtrItem>(conn)
//         .expect("Error loading round_meta_ptr")
// }


///////////////////
/// 
/// 
/// 
pub fn insert_round_meta_ptrs(conn: &mut PgConnection, data: Vec<RoundMetaPtrItem>) {
    diesel::insert_into(round_meta_ptrs::table)
        .values(&data)
        .execute(conn)
        .expect("Error saving new round meta ptr");
}

pub fn new_round_meta_ptrs(conn: &mut PgConnection, data: Vec<RoundMetaPtr>) {

    let mut round_meta_ptrs_arr: Vec<RoundMetaPtrItem> = Vec::new();
    for round_meta_ptr in data {
        let round_meta_ptr_item = RoundMetaPtrItem {
            id: round_meta_ptr.roundMetaPtr.id,
            protocol: round_meta_ptr.roundMetaPtr.protocol.to_string(),
            pointer: round_meta_ptr.roundMetaPtr.pointer,
            roundId: round_meta_ptr.id,
            chainId: round_meta_ptr.chainId.unwrap_or_default().to_string(),
        };

        round_meta_ptrs_arr.push(round_meta_ptr_item);
    }

    insert_round_meta_ptrs(conn, round_meta_ptrs_arr);
}


pub  fn insert_voting_strategies(conn: &mut PgConnection, data: Vec<VotingStrategyItem>) {
    diesel::insert_into(voting_strategies::table)
        .values(&data)
        .execute(conn)
        .expect("Error saving new voting strategy");
}

pub  fn new_voting_strategies(conn: &mut PgConnection, data: Vec<VotingStrategy>) {

    let mut voting_strategies: Vec<VotingStrategyItem> = Vec::new();
    for strategy in data {
        let voting_strategy_item = VotingStrategyItem {
            id: strategy.id, 
            strategyAddress: strategy.strategyAddress,
            strategyName: strategy.strategyName,
            version: strategy.version,
            chainId: strategy.chainId.unwrap_or_else(|| 99).to_string(),
            roundId: strategy.round.unwrap_or_else(|| utils::DerivedRoundId { id: "".to_string() }).id,
        };

        voting_strategies.push(voting_strategy_item);

    }

    let chunk_size = 1000;
    let mut voting_strategies_data = voting_strategies;

    while voting_strategies_data.len() > chunk_size {
        let (chunk, rest) = voting_strategies_data.split_at(chunk_size);
        insert_voting_strategies(conn, chunk.to_vec());
        voting_strategies_data = rest.to_vec()
    }

    insert_voting_strategies(conn, voting_strategies_data);

}

pub fn insert_round_projects_meta_ptrs(conn: &mut PgConnection, data: Vec<RoundProjectsMetaPtrItem>) {
    diesel::insert_into(round_projects_meta_ptrs::table)
        .values(&data)
        .execute(conn)
        .expect("Error saving new round projects meta ptr");
}

pub fn new_round_projects_meta_ptrs(conn: &mut PgConnection, data: Vec<RoundProjectsMetaPtr>) {

    let mut round_projects_meta_ptrs: Vec<RoundProjectsMetaPtrItem> = Vec::new();
    for round_projects_meta_ptr in data {
        let round_projects_meta_ptr_item = RoundProjectsMetaPtrItem {
            id: round_projects_meta_ptr.projectsMetaPtr.id,
            protocol: round_projects_meta_ptr.projectsMetaPtr.protocol.to_string(),
            pointer: round_projects_meta_ptr.projectsMetaPtr.pointer,
            roundId: round_projects_meta_ptr.id,
            chainId: round_projects_meta_ptr.chainId.unwrap_or_default().to_string(),
        };

        round_projects_meta_ptrs.push(round_projects_meta_ptr_item);
    }

    let chunk_size = 1000;
    let mut round_projects_meta_ptrs_data = round_projects_meta_ptrs;

    while round_projects_meta_ptrs_data.len() > chunk_size {
        let (chunk, rest) = round_projects_meta_ptrs_data.split_at(chunk_size);
        insert_round_projects_meta_ptrs(conn, chunk.to_vec());
        round_projects_meta_ptrs_data = rest.to_vec()
    }

    insert_round_projects_meta_ptrs(conn, round_projects_meta_ptrs_data);
}

pub fn insert_rounds1(conn: &mut PgConnection, data: Vec<RoundItem>) {
    diesel::insert_into(rounds::table)
        .values(&data)
        .execute(conn)
        .expect("Error saving new round");
}

pub fn new_rounds1(conn: &mut PgConnection, data: Vec<Round1>) {

    let mut round_items: Vec<RoundItem> = Vec::new();
    for round in data {
        let round_item = RoundItem {
            id: round.id,
            payoutStrategy: round.payoutStrategy,
            token: round.token,
            roundStartTime: round.roundStartTime,
            roundEndTime: round.roundEndTime,
            applicationsStartTime: round.applicationsStartTime,
            applicationsEndTime: round.applicationsEndTime,
            createdAt: round.createdAt,
            updatedAt: round.updatedAt,
            chainId: Some(round.chainId.unwrap_or_default().to_string()),
        };

        round_items.push(round_item);

    }

    let chunk_size = 1000;
    let mut rounds_data = round_items;

    while rounds_data.len() > chunk_size {
        let (chunk, rest) = rounds_data.split_at(chunk_size);
        insert_rounds1(conn, chunk.to_vec());
        rounds_data = rest.to_vec()
    }

    insert_rounds1(conn, rounds_data);

}

pub fn insert_project_meta_ptrs1(conn: &mut PgConnection, data: Vec<ProjectMetaPtrItem>) {
    diesel::insert_into(project_meta_ptrs::table)
        .values(&data)
        .execute(conn)
        .expect("Error saving new project meta ptr");
}

pub fn new_project_meta_ptrs1(conn: &mut PgConnection, data: Vec<ProjectMetaPtr>) {

    let mut project_meta_ptrs: Vec<ProjectMetaPtrItem> = Vec::new();
    for project_meta_ptr in data {
        let project_id = project_meta_ptr.id.split("-").collect::<Vec<&str>>()[0].to_string();
        let project_meta_ptr_item = ProjectMetaPtrItem {
            id: project_meta_ptr.id,
            protocol: project_meta_ptr.metaPtr.protocol.to_string(),
            pointer: project_meta_ptr.metaPtr.pointer,
            roundId: project_meta_ptr.round.id,
            projectId: project_id,
            chainId: project_meta_ptr.chainId.unwrap_or_default().to_string(),
        };

        project_meta_ptrs.push(project_meta_ptr_item);
    }

    let chunk_size = 1000;
    let mut project_meta_ptrs_data = project_meta_ptrs;

    while project_meta_ptrs_data.len() > chunk_size {
        let (chunk, rest) = project_meta_ptrs_data.split_at(chunk_size);
        insert_project_meta_ptrs1(conn, chunk.to_vec());
        project_meta_ptrs_data = rest.to_vec()
    }

    insert_project_meta_ptrs1(conn, project_meta_ptrs_data);
}

pub fn insert_projects1(conn: &mut PgConnection, data: Vec<ProjectItem>) {
    diesel::insert_into(projects::table)
        .values(&data)
        .execute(conn)
        .expect("Error saving new project");
}

pub fn new_projects1(conn: &mut PgConnection, data: Vec<Project2>) {

    let mut project_items: Vec<ProjectItem> = Vec::new();
    for project2 in data {
        let project_item = ProjectItem {
            id: project2.id,
            payoutAddress: project2.payoutAddress,
            projectId: project2.project,
            createdAt: project2.createdAt,
            updatedAt: project2.updatedAt,
            chainId: project2.chainId.unwrap().to_string(),
            roundId: project2.round.id,
        };

        project_items.push(project_item);

    }

    let chunk_size = 1000;
    let mut projects_data = project_items;

    while projects_data.len() > chunk_size {
        let (chunk, rest) = projects_data.split_at(chunk_size);
        insert_projects1(conn, chunk.to_vec());
        projects_data = rest.to_vec()
    }

    insert_projects1(conn, projects_data);

}

pub fn insert_qf_votes1(conn: &mut PgConnection, data: Vec<QfVoteItem>) {
    diesel::insert_into(qf_votes::table)
        .values(&data)
        .on_conflict(qf_votes::id)
        .do_nothing()
        .execute(conn)
        .expect("Error saving new qf vote");
}

pub fn new_qf_votes1(conn: &mut PgConnection, data: Vec<QfVote>) {

    
    let mut qf_votes: Vec<QfVoteItem> = Vec::new();
    for qf_vote in data {
        let qf_vote_item = QfVoteItem {
            id: qf_vote.id,
            amount: qf_vote.amount,
            from: qf_vote.from,
            to: qf_vote.to,
            projectId: qf_vote.projectId,
            token: qf_vote.token,
            version: qf_vote.version,
            createdAt: qf_vote.createdAt,
            roundId: qf_vote.votingStrategy.round.unwrap_or_else(|| utils::QfVotesDerivedRoundId { id: "".to_string() }).id,
            chainId: qf_vote.chainId.unwrap_or_default().to_string(),
        };
        
        qf_votes.push(qf_vote_item);
    }
    
    let chunk_size = 1000;

    let mut qf_votes_data = qf_votes;

    while qf_votes_data.len() > chunk_size {
        let (chunk, rest) = qf_votes_data.split_at(chunk_size);
        insert_qf_votes1(conn, chunk.to_vec());
        qf_votes_data = rest.to_vec()
    }

    insert_qf_votes1(conn, qf_votes_data);

    // insert_qf_votes1(conn, qf_votes);
}

pub async fn get_round_meta_ptr(conn: &mut PgConnection, round_id: String) -> Vec<RoundMetaPtrItem> {
    use crate::schema::round_meta_ptrs::dsl::*;

    let round_meta_ptr = round_meta_ptrs
        .filter(roundId.eq(round_id))
        .load::<RoundMetaPtrItem>(conn)
        .expect("Error loading round meta ptr");

    round_meta_ptr
}

pub async fn get_round_projects_meta_ptr(conn: &mut PgConnection, round_id: String) -> Vec<RoundProjectsMetaPtrItem> {
    use crate::schema::round_projects_meta_ptrs::dsl::*;

    let round_projects_meta_ptr = round_projects_meta_ptrs
        .filter(roundId.eq(round_id))
        .load::<RoundProjectsMetaPtrItem>(conn)
        .expect("Error loading round projects meta ptr");

    round_projects_meta_ptr
}

pub async fn get_round_data(conn: &mut PgConnection, round_id: String) -> Vec<RoundItem> {
    use crate::schema::rounds::dsl::*;

    let round = rounds
        .filter(id.eq(round_id))
        .load::<RoundItem>(conn)
        .expect("Error loading round");

    round
}

pub async fn get_round_voting_strategy(conn: &mut PgConnection, round_id: String) -> Vec<VotingStrategyItem> {
    use crate::schema::voting_strategies::dsl::*;

    let round_voting_strategy = voting_strategies
        .filter(roundId.eq(round_id))
        .load::<VotingStrategyItem>(conn)
        .expect("Error loading round voting strategy");

    round_voting_strategy
}

pub async fn get_round_projects(conn: &mut PgConnection, round_id: String) -> Vec<ProjectItem> {
    use crate::schema::projects::dsl::*;

    let round_projects = projects
        .filter(roundId.eq(round_id))
        .load::<ProjectItem>(conn)
        .expect("Error loading round projects");

    round_projects
}

pub async fn get_round_votes(conn: &mut PgConnection, round_id: String) -> Vec<QfVoteItem> {
    use crate::schema::qf_votes::dsl::*;

    let round_qf_votes = qf_votes
        .filter(roundId.eq(round_id))
        .load::<QfVoteItem>(conn)
        .expect("Error loading round qf votes");

    round_qf_votes
}

pub async fn get_project_data(conn: &mut PgConnection, project_id: String) -> Vec<ProjectItem> {
    use crate::schema::projects::dsl::*;

    let project_data = projects
        .filter(projectId.eq(project_id))
        .load::<ProjectItem>(conn)
        .expect("Error loading project");

    project_data
}

pub async fn get_project_meta_ptr(conn: &mut PgConnection, project_id: String) -> Vec<ProjectMetaPtrItem> {
    use crate::schema::project_meta_ptrs::dsl::*;

    let project_meta_ptr = project_meta_ptrs
        .filter(projectId.eq(project_id))
        .load::<ProjectMetaPtrItem>(conn)
        .expect("Error loading project meta ptr");

    project_meta_ptr
}

pub async fn get_project_votes(conn: &mut PgConnection, project_id: String) -> Vec<QfVoteItem> {
    use crate::schema::qf_votes::dsl::*;

    let project_qf_votes = qf_votes
        .filter(projectId.eq(project_id))
        .load::<QfVoteItem>(conn)
        .expect("Error loading project qf votes");

    project_qf_votes
}