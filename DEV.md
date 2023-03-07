## Development

### Requirements

- Rust 1.54 or higher
- PostgreSQL 12 or higher
- Git
- Docker
- Diesel CLI

#### Database setup


Pull the latest PostgreSQL docker container. 

```shell
docker pull postgres
```

Run the docker container as a local temporary dev instance.

```shell 
docker run --rm -P -p 127.0.0.1:5432:5432 -e POSTGRES_PASSWORD="1234" --name pg postgres
```

Clone the repository. 

```shell
git clone https://github.com/gitcoinco/grants-stack-api.git && cd grants-stack-api
```


Create `.env` from the example template in the repo.

```shell
cp .env.example .env
```

Modify the `.env`'s `DATABASE_URL` postgres container's connection URL. In this case, we'll use the local instance we started above. 

```bash
...
DATABASE_URL=postgresql://postgres:1234@localhost:5432/postgres
...
```

Fill out the other environment variables in the `.env` as necessary. 

The local database is now prepared to be initialized with our table data.

Begin the table migration. 

```shell
diesel migrations run
```

This applies the `up.sql` migration schema in the latest `migration_*` directory, generating the diesel rust typings.

To re-apply a migration run

```shell
diesel migrations redo
```

Note: this will apply the `down.sql` schema. 

To generate a new migration 

```shell
diesel migration generate <NAME_OF_MIGRATION>
```

You must fill out the `up.sql` and `down.sql` schema before running the migration!

#### Code

To build and run the server locally

```shell
cargo run 
```

This will start the actix web-sever locally, and if connected to a database, it can receive and serve data.

Or, you can run the deployment docker containers.

```shell 
docker-compose up 
```

This will start the actix web server in a docker container, proxied by an nginx container.
If connected to a database, it can receive and serve data.

There is now an instance of the grants stack api running and ready to modify and test!

#### Test

Coming soon...