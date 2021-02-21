FROM rust:latest as reversi-server

RUN apt install git -y
RUN git clone https://github.com/marunaka610/reversi-server.git
WORKDIR /reversi-server


COPY ./boot.sh /
RUN chmod 777 /boot.sh

CMD [ "cargo run" ]

EXPOSE 8080


# # ビルド環境
# FROM reversi-server as build-stage
# RUN cargo build --release

# # 本番環境
# FROM rust:1.43.1-slim-stretch
# COPY --from=build-stage /app/target/release/api .
# EXPOSE 8088
# CMD ["/usr/local/bin/api"]
