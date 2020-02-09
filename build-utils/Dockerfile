FROM rust:latest

RUN apt-get update
RUN apt-get install -y clang
RUN apt-get clean

ADD entrypoint.sh /opt/entrypoint.sh
RUN chmod +x /opt/entrypoint.sh

ENTRYPOINT ["/opt/entrypoint.sh"]
