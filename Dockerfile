FROM clux/muslrust as backend_builder
WORKDIR /opt/cdb-transformer
COPY Cargo.* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch
COPY src/*.rs src/
RUN cargo build --release

FROM alpine
WORKDIR /opt/cdb-transformer
COPY --from=backend_builder /opt/cdb-transformer/target/x86_64-unknown-linux-musl/release/cdb-transformer .
ENTRYPOINT ["./cdb-transformer"]
