FROM rust:1.68

WORKDIR /app

RUN apt-get update
RUN apt-get install -y cmake

RUN cargo install sqlx-cli

CMD ["tail", "-f", "/dev/null"]