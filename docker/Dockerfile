FROM rust:latest

WORKDIR /usr/src/vmm

COPY . .

RUN cargo build --release

CMD ["./docker/startup-script.sh"]