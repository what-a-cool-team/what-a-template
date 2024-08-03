FROM rust:1.79 AS build
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release --package app

FROM debian:bookworm-slim
WORKDIR /opt/what-a-cool-team
COPY --from=build /usr/src/app/target/release/app /opt/what-a-cool-team/app
COPY settings.toml settings.toml
ENTRYPOINT ["/opt/what-a-cool-team/app", "--config", "/opt/what-a-cool-team/settings.toml"]
