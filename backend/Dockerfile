FROM rust:1.95-alpine3.23 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:3.23 AS run

WORKDIR /app

COPY --from=builder /app/target/release/task_manager /app

EXPOSE 8080

ENTRYPOINT [ "/app/task_manager" ]
