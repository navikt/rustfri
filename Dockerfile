FROM clux/muslrust:stable as builder
WORKDIR /build
COPY . .
ENV RUSTFLAGS='-C target-feature=+crt-static'
RUN cargo build --release

FROM gcr.io/distroless/static-debian11:nonroot
WORKDIR /
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/rustfri /
EXPOSE 8080
CMD ["/rustfri"]