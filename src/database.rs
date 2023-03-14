use diesel::{PgConnection, QueryDsl, RunQueryDsl};

use diesel::ExpressionMethods;

use crate::models::{
    DerivedRoundId, Project, ProjectItem, ProjectMetaPtr, ProjectMetaPtrItem, QfVote, QfVoteItem,
    Round, RoundItem, RoundMetaPtr, RoundMetaPtrItem, RoundProjectsMetaPtr,
    RoundProjectsMetaPtrItem, TokenPrice, TokenPriceItem, VotingStrategy, VotingStrategyItem,
};
use crate::schema::{
    project_meta_ptrs, projects, qf_votes, round_meta_ptrs, round_projects_meta_ptrs, rounds,
    token_prices, voting_strategies,
};

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

pub fn insert_voting_strategies(conn: &mut PgConnection, data: Vec<VotingStrategyItem>) {
    diesel::insert_into(voting_strategies::table)
        .values(&data)
        .execute(conn)
        .expect("Error saving new voting strategy");
}

pub fn new_voting_strategies(conn: &mut PgConnection, data: Vec<VotingStrategy>) {
    let mut voting_strategies: Vec<VotingStrategyItem> = Vec::new();
    for strategy in data {
        let voting_strategy_item = VotingStrategyItem {
            id: strategy.id,
            strategyAddress: strategy.strategyAddress,
            strategyName: strategy.strategyName,
            version: strategy.version,
            chainId: strategy.chainId.unwrap_or_else(|| 99).to_string(),
            roundId: strategy
                .round
                .unwrap_or_else(|| DerivedRoundId { id: "".to_string() })
                .id,
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

pub fn insert_round_projects_meta_ptrs(
    conn: &mut PgConnection,
    data: Vec<RoundProjectsMetaPtrItem>,
) {
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
            chainId: round_projects_meta_ptr
                .chainId
                .unwrap_or_default()
                .to_string(),
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

pub fn insert_rounds(conn: &mut PgConnection, data: Vec<RoundItem>) {
    diesel::insert_into(rounds::table)
        .values(&data)
        .execute(conn)
        .expect("Error saving new round");
}

pub fn new_rounds(conn: &mut PgConnection, data: Vec<Round>) {
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
        insert_rounds(conn, chunk.to_vec());
        rounds_data = rest.to_vec()
    }

    insert_rounds(conn, rounds_data);
}

pub fn insert_project_meta_ptrs(conn: &mut PgConnection, data: Vec<ProjectMetaPtrItem>) {
    diesel::insert_into(project_meta_ptrs::table)
        .values(&data)
        .execute(conn)
        .expect("Error saving new project meta ptr");
}

pub fn new_project_meta_ptrs(conn: &mut PgConnection, data: Vec<ProjectMetaPtr>) {
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
        insert_project_meta_ptrs(conn, chunk.to_vec());
        project_meta_ptrs_data = rest.to_vec()
    }

    insert_project_meta_ptrs(conn, project_meta_ptrs_data);
}

pub fn insert_projects(conn: &mut PgConnection, data: Vec<ProjectItem>) {
    diesel::insert_into(projects::table)
        .values(&data)
        .execute(conn)
        .expect("Error saving new project");
}

pub fn new_projects(conn: &mut PgConnection, data: Vec<Project>) {
    let mut project_items: Vec<ProjectItem> = Vec::new();
    for project in data {
        let project_item = ProjectItem {
            id: project.id,
            payoutAddress: project.payoutAddress,
            projectId: project.project,
            createdAt: project.createdAt,
            updatedAt: project.updatedAt,
            chainId: project.chainId.unwrap().to_string(),
            roundId: project.round.id,
        };

        project_items.push(project_item);
    }

    let chunk_size = 1000;
    let mut projects_data = project_items;

    while projects_data.len() > chunk_size {
        let (chunk, rest) = projects_data.split_at(chunk_size);
        insert_projects(conn, chunk.to_vec());
        projects_data = rest.to_vec()
    }

    insert_projects(conn, projects_data);
}

pub fn insert_qf_votes(conn: &mut PgConnection, data: Vec<QfVoteItem>) {
    diesel::insert_into(qf_votes::table)
        .values(&data)
        .on_conflict(qf_votes::id)
        .do_nothing()
        .execute(conn)
        .expect("Error saving new qf vote");
}

pub fn new_qf_votes(conn: &mut PgConnection, data: Vec<QfVote>) {
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
            roundId: qf_vote
                .votingStrategy
                .round
                .unwrap_or_else(|| DerivedRoundId { id: "".to_string() })
                .id,
            chainId: qf_vote.chainId.unwrap_or_default().to_string(),
        };

        qf_votes.push(qf_vote_item);
    }

    let chunk_size = 1000;

    let mut qf_votes_data = qf_votes;

    while qf_votes_data.len() > chunk_size {
        let (chunk, rest) = qf_votes_data.split_at(chunk_size);
        insert_qf_votes(conn, chunk.to_vec());
        qf_votes_data = rest.to_vec()
    }

    insert_qf_votes(conn, qf_votes_data);
}

pub fn insert_token_price(conn: &mut PgConnection, data: Vec<TokenPriceItem>) {
    diesel::insert_into(token_prices::table)
        .values(&data)
        .execute(conn)
        .expect("Error saving new token price");
}

pub fn new_token_prices(
    conn: &mut PgConnection,
    data: Vec<TokenPrice>,
    token: String,
    chainId: String,
) {
    let mut token_prices: Vec<TokenPriceItem> = Vec::new();
    for token_price in data {
        let token_price_item = TokenPriceItem {
            timestamp: token_price.timestamp.to_string(),
            price: token_price.usd.to_string(),
            token: token.clone(),
            chainId: chainId.clone(),
        };

        token_prices.push(token_price_item);
    }

    let chunk_size = 1000;
    let mut token_prices_data = token_prices;

    while token_prices_data.len() > chunk_size {
        let (chunk, rest) = token_prices_data.split_at(chunk_size);
        insert_token_price(conn, chunk.to_vec());
        token_prices_data = rest.to_vec()
    }

    insert_token_price(conn, token_prices_data);
}

pub async fn get_round_meta_ptr(
    conn: &mut PgConnection,
    round_id: String,
) -> Vec<RoundMetaPtrItem> {
    use crate::schema::round_meta_ptrs::dsl::*;

    let round_meta_ptr = round_meta_ptrs
        .filter(roundId.eq(round_id))
        .load::<RoundMetaPtrItem>(conn)
        .expect("Error loading round meta ptr");

    round_meta_ptr
}

pub async fn get_round_projects_meta_ptr(
    conn: &mut PgConnection,
    round_id: String,
) -> Vec<RoundProjectsMetaPtrItem> {
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

pub async fn get_round_voting_strategy(
    conn: &mut PgConnection,
    round_id: String,
) -> Vec<VotingStrategyItem> {
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

pub async fn get_project_meta_ptr(
    conn: &mut PgConnection,
    project_id: String,
) -> Vec<ProjectMetaPtrItem> {
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
