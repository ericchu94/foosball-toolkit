FROM docker.io/library/rust:alpine

RUN apk add --no-cache musl-dev file

RUN rustup default nightly

COPY . /build

WORKDIR /build

RUN SQLX_OFFLINE=true cargo build --release --bin foos-db

RUN file target/release/foos-db && ldd target/release/foos-db

FROM scratch

COPY --from=0 /build/target/release/foos-db /

ENTRYPOINT ["/foos-db"]
