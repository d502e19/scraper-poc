FROM rust:1.37

WORKDIR /usr/src/poc
COPY . .

RUN cargo install --path .

CMD ["poc"]