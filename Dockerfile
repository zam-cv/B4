FROM rust:1.76.0 AS builder

ENV MODE=dev

WORKDIR /usr/src/app

COPY backend/assets ./assets
COPY backend/macros ./macros
COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/build.rs .

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

RUN rm -f target/release/deps/backend*

COPY backend/src ./src

RUN cargo build --release

RUN apt-get update && \
    apt-get install -y libmariadb-dev-compat libmariadb-dev && \
    rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --no-default-features --features mysql

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y libmariadb-dev-compat libmariadb-dev wget openssh-server && \
    rm -rf /var/lib/apt/lists/*

RUN mkdir /var/run/sshd

COPY ./oci_instance.pub /root/.ssh/authorized_keys
RUN sed -i 's/#PermitRootLogin yes/PermitRootLogin yes/' /etc/ssh/sshd_config
RUN sed -i 's/#PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config
RUN sed -i 's/#PubkeyAuthentication yes/PubkeyAuthentication yes/' /etc/ssh/sshd_config
RUN echo "AuthorizedKeysFile /root/.ssh/authorized_keys" >> /etc/ssh/sshd_config

RUN chmod 600 /root/.ssh/authorized_keys

COPY --from=builder /usr/src/app/target/release/backend /usr/local/bin/app
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

ENV DOCKERIZE_VERSION v0.6.1

RUN wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz && \
    tar -C /usr/local/bin -xzvf dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz && \
    rm dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz

WORKDIR /usr/src/app
COPY page ./page

WORKDIR /usr/src/app/backend

COPY backend/cert ./cert
COPY backend/migrations ./migrations
COPY backend/diesel.toml .
COPY backend/build.rs .

EXPOSE 8080
EXPOSE 22

COPY entrypoint.sh /usr/src/app/entrypoint.sh
RUN chmod +x /usr/src/app/entrypoint.sh

ENTRYPOINT ["/usr/src/app/entrypoint.sh"]
CMD ["app"]