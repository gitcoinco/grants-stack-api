## Grants Stack API

### Requirements

- Rust 1.54 or higher
- PostgreSQL 12 or higher
- Git
- Docker
- Diesel CLI

### Getting started

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

#### `GET /seed/{chain_id}`

This endpoint triggers data seeding for the specified chain ID. It accepts a path parameter `chain_id`, and returns a response indicating whether the seeding was successful or not.

#### `GET /round`

This endpoint fetches round data from the database. It accepts a query parameter `round_id` specifying the round to fetch data for, and optional boolean parameters indicating which pieces of data to include in the response. Multiple parameters can be used at once.

##### Parameter - Description

- `data`: Include round data in the response.
- `round_meta_ptr`: Include round metadata pointer in the response.
- `voting_strategy`: Include voting strategy data in the response.
- `projects_meta_ptr`: Include projects metadata pointer in the response.
- `round_projects`: Include round projects data in the response.
- `round_votes`: Include round votes data in the response.

#### `GET /project`

This endpoint fetches project data from the database. It accepts a query parameter `project_id` specifying the project to fetch data for, and optional boolean parameters indicating which pieces of data to include in the response. Multiple parameters can be used at once.

##### Parameter - Description

- `data`: Include project data in the response.
- `project_meta_ptr`: Include project metadata pointer in the response.
- `project_votes`: Include project votes data in the response.
- `project_summary`: Include project summary data in the response.

#### `GET /ipfs`

This endpoint relays an IPFS query. It accepts a query parameter cid specifying the content ID to query. The response returns the queried data in JSON format. Caching is not yet implemented.
