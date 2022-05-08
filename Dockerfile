FROM rust:1.60.0 as build-env
WORKDIR /app
COPY . /app
RUN rustup toolchain install nightly
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/uwurandom-as-a-service /
CMD ["./uwurandom-as-a-service"]
