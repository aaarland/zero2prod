FROM rust:latest
WORKDIR /app
# RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release
ENV APP_ENVIORNMENT production
ENTRYPOINT [ "./target/release/zero2prod" ]