sed -i 's:# define IPPORT_RESERVED:// #define IPPORT_RESERVED:' /usr/include/netdb.h
apt-get update
apt-get install -y clang
cd /nginx-rs
cargo build
