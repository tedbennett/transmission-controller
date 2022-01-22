FROM rust:1.56.1
WORKDIR /usr/src/app
COPY . .
EXPOSE 8000
RUN cargo build

CMD ["cargo", "run"]