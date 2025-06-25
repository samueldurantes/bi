## Build tools & versions used

- **Rust 2024 edition**, with crates like `axum`, `tokio`, and `sqlx` for async web and database work.
- **Docker** and **docker-compose** to easily spin up the environment.
- **Makefile** to simplify common dev tasks and setup steps.
- Handy libraries like `anyhow`, `thiserror`, and `tower-http` to make error handling and observability more solid.

## Steps to run the app

### Prerequisites

- [Docker](https://www.docker.com/)
- [docker-compose](https://docs.docker.com/compose/)
- [sqlx-cli](https://crates.io/crates/sqlx-cli)
- [Rust & Cargo](https://www.rust-lang.org/tools/install)

#### 1. Setup environment and start containers

This will check for `docker-compose`, copy the `.env.example` file, and start the containers:

```bash
make setup
```

#### 2. Run database migrations

This will check for the `sqlx` CLI and set up the database:

```bash
make migrate
```

OBS: If you run the commands too quickly, you might get an error, it’s likely because the database hasn’t fully initialized in Docker yet.

#### 3. Run the application

This will start the application in development mode:

```bash
make run-dev
```

The app will be available at `http://localhost:8080`, if you don't change the port in the `.env` file.

## What was the reason for your focus? What problems were you trying to solve?

My main focus was to make the API as simple and safe as possible.

I spent some time making sure most potential errors are properly handled, so the process doesn’t crash during execution.

## How long did you spend on this project?

I spent about 4 hours on this project.

![wakatime](https://files.catbox.moe/3irv6w.png)

## Did you make any trade-offs for this project? What would you have done differently with more time?

Yes, I made some trade-offs. If I had more time, I would improve the sync process to avoid inserting all node records
every time. Instead, I would only add new records or update the ones that changed. This way, it would be more efficient
and avoid unnecessary database work.

Another trade-off was keeping the insertion logic simple, without implementing batching. PostgreSQL has a practical limit
on the number of rows per INSERT statement (typically around 1000 rows), and the current implementation does not account for that.
Ideally, I would batch the insertions in chunks to ensure compatibility with large datasets. However, since pagination or data
limits were not part of the initial requirements, I chose to prioritize simplicity over robustness in this case.

## What do you think is the weakest part of your project?

The weakest part of my project is that it doesn’t check if any record from the original API has disappeared. In other
words, once a record is added, it will always show up in the results from my API, even if it was removed from the
original source.

## Is there any other information you’d like us to know?

Yes, I added automated tests to ensure correctness and reliability. A CI/CD pipeline was also set up using GitHub Actions to automate testing and
deployment. The final build is deployed on my personal homelab, making it easy to test and iterate quickly.

You can test the live deployment by running:

```bash
curl https://bi.samueldurante.com/nodes
```
