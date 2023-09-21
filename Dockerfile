FROM rust:1.72.0 as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /
COPY --from=build-env /app/target/release/rustfri /
EXPOSE 8080
CMD ["/rustfri"]