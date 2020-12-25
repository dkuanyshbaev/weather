FROM ubuntu:20.10

WORKDIR /app

COPY ./target/release/weather /app

EXPOSE 4444

CMD ["./weather"]
