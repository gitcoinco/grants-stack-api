## First stage: Build the application
FROM rust:latest as builder
# create a new empty shell project
RUN USER=root cargo new --bin grants-stack-api
# set the working directory
WORKDIR ./grants-stack-api
# copy the source code
ADD . ./
# build the project
RUN cargo build --release

## Second stage: Create the final image
FROM debian:buster-slim
ARG APP=/usr/src/app
# install dependencies
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libpq-dev \
    && rm -rf /var/lib/apt/lists/*
# set the timezone & user env variables
ENV TZ=Etc/UTC \
    APP_USER=appuser
# create a user and a group for the application
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}
# copy binaries from the first stage
COPY --from=builder /grants-stack-api/target/release/grants-stack-api ${APP}/grants-stack-api
RUN chown -R $APP_USER:$APP_USER ${APP}
# set the user
USER $APP_USER
# set the working directory
WORKDIR ${APP}
# expose the port
EXPOSE 8080
# run the binary
CMD [ "sh", "-c", "./grants-stack-api migrate && ./grants-stack-api serve" ]
