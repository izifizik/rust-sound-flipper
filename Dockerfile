FROM rust:1.82-slim-bullseye

RUN apt-get update && apt-get install -y \
    libmp3lame-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY . .
RUN cargo fetch

RUN cargo build --release

CMD ["cargo", "run", "--release"]
