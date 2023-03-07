// @generated automatically by Diesel CLI.

diesel::table! {
    programs (id) {
        id -> Text,
        createdAt -> Text,
        updatedAt -> Text,
        chainId -> Nullable<Text>,
    }
}

diesel::table! {
    project_matches (id) {
        id -> Int4,
        createdAt -> Text,
        updatedAt -> Text,
        projectId -> Text,
        roundId -> Text,
        chainId -> Nullable<Text>,
    }
}

diesel::table! {
    project_meta_ptrs (id) {
        id -> Text,
        protocol -> Nullable<Text>,
        pointer -> Nullable<Text>,
        roundId -> Nullable<Text>,
        chainId -> Nullable<Text>,
    }
}

diesel::table! {
    project_summaries (id) {
        id -> Int4,
        createdAt -> Text,
        updatedAt -> Text,
        projectId -> Text,
        roundId -> Text,
        chainId -> Text,
        totalVotesInUSDC -> Text,
        uniqueVoters -> Text,
        totalVoters -> Text,
    }
}

diesel::table! {
    projects (id) {
        id -> Text,
        payoutAddress -> Nullable<Text>,
        project -> Text,
        createdAt -> Text,
        updatedAt -> Text,
        chainId -> Nullable<Text>,
        roundId -> Nullable<Text>,
    }
}

diesel::table! {
    projects_meta_ptrs (id) {
        id -> Text,
        protocol -> Nullable<Text>,
        pointer -> Nullable<Text>,
        createdAt -> Nullable<Text>,
        updatedAt -> Nullable<Text>,
        roundId -> Nullable<Text>,
    }
}

diesel::table! {
    qf_votes (id) {
        id -> Text,
        createdAt -> Text,
        amount -> Text,
        from -> Text,
        to -> Text,
        projectId -> Text,
        token -> Text,
        version -> Text,
        chainId -> Nullable<Text>,
        roundId -> Nullable<Text>,
    }
}

diesel::table! {
    round_meta_ptrs (id) {
        id -> Text,
        protocol -> Text,
        pointer -> Text,
        roundId -> Text,
        chainId -> Text,
    }
}

diesel::table! {
    round_projects_meta_ptrs (id) {
        id -> Text,
        protocol -> Nullable<Text>,
        pointer -> Nullable<Text>,
        roundId -> Nullable<Text>,
        chainId -> Nullable<Text>,
    }
}

diesel::table! {
    rounds (id) {
        id -> Text,
        payoutStrategy -> Text,
        token -> Text,
        roundStartTime -> Text,
        roundEndTime -> Text,
        applicationsStartTime -> Text,
        applicationsEndTime -> Text,
        createdAt -> Text,
        updatedAt -> Text,
        chainId -> Nullable<Text>,
    }
}

diesel::table! {
    votes (id) {
        id -> Text,
        createdAt -> Text,
        amount -> Text,
        from -> Text,
        to -> Text,
        token -> Text,
        version -> Text,
        projectId -> Nullable<Text>,
        chainId -> Nullable<Text>,
    }
}

diesel::table! {
    voting_strategies (id) {
        id -> Text,
        strategyAddress -> Nullable<Text>,
        strategyName -> Nullable<Text>,
        version -> Nullable<Text>,
        roundId -> Nullable<Text>,
        chainId -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    programs,
    project_matches,
    project_meta_ptrs,
    project_summaries,
    projects,
    projects_meta_ptrs,
    qf_votes,
    round_meta_ptrs,
    round_projects_meta_ptrs,
    rounds,
    votes,
    voting_strategies,
);
