FROM rust:latest

RUN apt-get update
RUN apt-get install -y clang
RUN apt-get clean

RUN sed -i 's:# define IPPORT_RESERVED:// #define IPPORT_RESERVED:' /usr/include/netdb.h

ADD entrypoint.sh /opt/entrypoint.sh
RUN chmod +x /opt/entrypoint.sh

ENTRYPOINT ["/opt/entrypoint.sh"]
