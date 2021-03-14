FROM rust:latest as reversi-server

RUN apt update ;\
  apt install git -y ;\
  apt install -y lsb-release ;\ 
  apt remove -y libmariadb-dev-compat libmariadb-dev ;\ 
  wget https://dev.mysql.com/get/Downloads/MySQL-8.0/mysql-common_8.0.18-1debian10_amd64.deb \
  https://dev.mysql.com/get/Downloads/MySQL-8.0/libmysqlclient21_8.0.18-1debian10_amd64.deb \
  https://dev.mysql.com/get/Downloads/MySQL-8.0/mysql-community-client-core_8.0.18-1debian10_amd64.deb \
  https://dev.mysql.com/get/Downloads/MySQL-8.0/mysql-community-client_8.0.18-1debian10_amd64.deb \
  https://dev.mysql.com/get/Downloads/MySQL-8.0/libmysqlclient-dev_8.0.18-1debian10_amd64.deb; \
  dpkg -i mysql-common_8.0.18-1debian10_amd64.deb \
  libmysqlclient21_8.0.18-1debian10_amd64.deb \
  mysql-community-client-core_8.0.18-1debian10_amd64.deb \
  mysql-community-client_8.0.18-1debian10_amd64.deb \
  libmysqlclient-dev_8.0.18-1debian10_amd64.deb ; \
  git clone https://github.com/marunaka610/reversi-server.git ;\
  cargo install cargo-watch 

WORKDIR /reversi-server


# COPY ./boot.sh /
# RUN chmod 777 /boot.sh

# CMD [ "/usr/local/cargo/bin/cargo run" ]

EXPOSE 8080


# # ビルド環境
# FROM reversi-server as build-stage
# RUN cargo build --release

# # 本番環境
# FROM rust:1.43.1-slim-stretch
# COPY --from=build-stage /app/target/release/api .
# EXPOSE 8088
# CMD ["/usr/local/bin/api"]
