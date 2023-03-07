CREATE TABLE programs (
    "id" TEXT NOT NULL,
    "createdAt" TEXT NOT NULL,
    "updatedAt" TEXT NOT NULL,
    "chainId" TEXT,
    CONSTRAINT "programs_pkey" PRIMARY KEY ("id")
);
CREATE TABLE rounds (
    "id" TEXT NOT NULL,
    "payoutStrategy" TEXT NOT NULL,
    "token" TEXT NOT NULL,
    "roundStartTime" TEXT NOT NULL,
    "roundEndTime" TEXT NOT NULL,
    "applicationsStartTime" TEXT NOT NULL,
    "applicationsEndTime" TEXT NOT NULL,
    "createdAt" TEXT NOT NULL,
    "updatedAt" TEXT NOT NULL,
    "chainId" TEXT,
    CONSTRAINT "rounds_pkey" PRIMARY KEY ("id")
);
CREATE TABLE projects (
    "id" TEXT NOT NULL,
    "payoutAddress" TEXT,
    "project" TEXT NOT NULL,
    "createdAt" TEXT NOT NULL,
    "updatedAt" TEXT NOT NULL,
    "chainId" TEXT NOT NULL,
    "roundId" TEXT NOT NULL,
    CONSTRAINT "projects_pkey" PRIMARY KEY ("id")
);
CREATE TABLE votes (
    "id" TEXT NOT NULL,
    "createdAt" TEXT NOT NULL,
    "amount" TEXT NOT NULL,
    "from" TEXT NOT NULL,
    "to" TEXT NOT NULL,
    "token" TEXT NOT NULL,
    "version" TEXT NOT NULL,
    "projectId" TEXT,
    "chainId" TEXT,
    CONSTRAINT "votes_pkey" PRIMARY KEY ("id")
);
CREATE TABLE project_matches (
    "id" SERIAL NOT NULL,
    "createdAt" TEXT NOT NULL,
    "updatedAt" TEXT NOT NULL,
    "projectId" TEXT NOT NULL,
    "roundId" TEXT NOT NULL,
    "chainId" TEXT,
    CONSTRAINT "project_matches_pkey" PRIMARY KEY ("id")
);
CREATE TABLE project_summaries (
    --     ids iterate
    "id" SERIAL NOT NULL,
    "createdAt" TEXT NOT NULL,
    "updatedAt" TEXT NOT NULL,
    "projectId" TEXT NOT NULL,
    "roundId" TEXT NOT NULL,
    "chainId" TEXT NOT NULL,
    "totalVotesInUSDC" TEXT NOT NULL,
    "uniqueVoters" TEXT NOT NULL,
    "totalVoters" TEXT NOT NULL,
    CONSTRAINT "project_summaries_pkey" PRIMARY KEY ("id")
);
CREATE TABLE projects_meta_ptrs (
    "id" TEXT NOT NULL,
    "protocol" TEXT,
    "pointer" TEXT,
    "createdAt" TEXT,
    "updatedAt" TEXT,
    "roundId" TEXT,

    CONSTRAINT "projects_meta_ptrs_pkey" PRIMARY KEY ("id")
);

CREATE TABLE voting_strategies (
    "id" TEXT NOT NULL,
    "strategyAddress" TEXT NOT NULL,
    "strategyName" TEXT NOT NULL,
    "version" TEXT NOT NULL,
    "roundId" TEXT NOT NULL,
    "chainId" TEXT NOT NULL,
    CONSTRAINT "voting_strategies_pkey" PRIMARY KEY ("id")
);

CREATE TABLE round_meta_ptrs (
    "id" TEXT NOT NULL,
    "protocol" TEXT NOT NULL,
    "pointer" TEXT NOT NULL,
    "roundId" TEXT NOT NULL,
    "chainId" TEXT NOT NULL,

    CONSTRAINT "round_meta_ptrs_pkey" PRIMARY KEY ("id")
);

CREATE TABLE round_projects_meta_ptrs (
    "id" TEXT NOT NULL,
    "protocol" TEXT NOT NULL,
    "pointer" TEXT NOT NULL,
    "roundId" TEXT NOT NULL,
    "chainId" TEXT NOT NULL,

    CONSTRAINT "round_projects_meta_ptrs_pkey" PRIMARY KEY ("id")
);

CREATE TABLE project_meta_ptrs (
    "id" TEXT NOT NULL,
    "protocol" TEXT,
    "pointer" TEXT,
    "roundId" TEXT,
    "chainId" TEXT,

    CONSTRAINT "project_meta_ptrs_pkey" PRIMARY KEY ("id")
); 

CREATE TABLE qf_votes (
    "id" TEXT NOT NULL,
    "createdAt" TEXT NOT NULL,
    "amount" TEXT NOT NULL,
    "from" TEXT NOT NULL,
    "to" TEXT NOT NULL,
    "projectId" TEXT NOT NULL,
    "token" TEXT NOT NULL,
    "version" TEXT NOT NULL,
    "chainId" TEXT NOT NULL,
    "roundId" TEXT NOT NULL,

    CONSTRAINT "qf_votes_pkey" PRIMARY KEY ("id")
)