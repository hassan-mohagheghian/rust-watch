FROM rust:1.75

WORKDIR /app

COPY . .

RUN cargo build -p collector

CMD ["cargo", "run", "-p", "collector"]