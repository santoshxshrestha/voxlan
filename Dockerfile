FROM rust:latest

WORKDIR /usr/src/voxlan
COPY . .

RUN cargo install --path .

CMD ["voxlan"]
