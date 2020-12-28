FROM ubuntu:20.10

RUN apt-get update
RUN apt-get -y install libssl-dev
RUN apt-get -y install openssl
RUN apt-get -y install ca-certificates

WORKDIR /app

COPY ./target/release/weather /app

EXPOSE 4444

CMD ["./weather"]
