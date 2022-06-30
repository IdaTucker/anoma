# cargo build --target x86_64-unknown-linux-gnu
# docker build -t tmp .
# docker run \
#     -p 127.0.0.1:8123:8123 \
#     -p 127.0.0.1:26656:26656 \
#     -p 127.0.0.1:26657:26657 \
#     -it \
#     --rm \
#     tmp
FROM ghcr.io/james-chf/devchain-container:sha-8e60590
COPY target/debug/anoma /usr/local/bin
COPY target/debug/anomac /usr/local/bin
COPY target/debug/anoman /usr/local/bin
COPY target/debug/anomaw /usr/local/bin
