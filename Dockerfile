FROM rust AS build

RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /build

COPY . /build

RUN cargo install --target=x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=build /usr/local/cargo/bin/get-ip /
USER 1000
ENTRYPOINT ["/get-ip"]
