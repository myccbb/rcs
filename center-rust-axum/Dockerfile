FROM ubuntu:rolling
RUN mkdir -p /etc/center
COPY ./config.toml /etc/center/config.toml
COPY ./target/release/center /usr/local/bin/center
CMD ["center" "--conf" "/etc/center/config.toml"]