FROM rust:1.73

COPY ./ ./

RUN cargo build --release

WORKDIR /target/release

CMD ["./todo_backend_rs"]
