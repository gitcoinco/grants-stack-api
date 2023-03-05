## Grants Stack API

### Requirements

- Rust 1.54 or higher
- PostgreSQL 12 or higher
- Git
- Docker
- Diesel CLI

### Setup

Clone the repo

```shell
git clone https://github.com/gitcoinco/grants-stack-api.git && cd grants-stack-api
```

Create `.env` from the example template.

```shell
cp .env.example .env
```

Configure the `.env`.

If you have a fresh postgres database, preform a migration to initialize the necessary tables in the database.

```shell
diesel migration run
```

Run the `docker-compose.yml`

```shell
docker compose up
```

### Endpoints

#### `/seed/{chain_id}`

An endpoint to trigger seeding the database with data from the specified chain_id.

##### Parameters

- `chain_id` (required): The chain id of the data to seed.

##### Response

- `200 OK`: Data seeding completed successfully.
- `400 Bad Request`: The specified chain id is not supported.

#### `/rounds`

An endpoint for getting all rounds.

##### Response

- `200 OK`: A JSON array of all rounds.

#### `/projects`

An endpoint for getting all projects.

##### Response

- `200 OK`: A JSON array of all projects.

#### `/votes`

An endpoint for getting votes.

##### Parameters

- `project_id` (optional): A comma-separated list of project ids to filter votes by.

##### Response

- `200 OK`: A JSON array of votes. If project_id is specified, only votes for the specified projects are returned.

#### `/ipfs`

An endpoint for relaying an IPFS query.

##### Parameters

- `cid` (required): The content ID of the IPFS data to fetch.

##### Response

- `200 OK`: The IPFS data as a JSON object.
