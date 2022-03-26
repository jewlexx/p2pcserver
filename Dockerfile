FROM --platform=arm64 rust:latest

COPY . /usr/src/app

WORKDIR /usr/src/app

RUN cargo build -r

EXPOSE 8000

CMD [ "/usr/src/app/target/release/p2pcserver" ]