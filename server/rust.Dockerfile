# Build stage
FROM rust:1.69-buster as builder

WORKDIR /app

ARG DATABASE_ARGS

ENV DATABASE_ARGS=$DATABASE_ARGS

COPY . .

RUN cargo build --release

#Preduction stage
FROM debian:buster-slim

WORKDIR /user/local/bin

COPY --from=builder /app/target/release/server .

CMD [ "./server" ]
