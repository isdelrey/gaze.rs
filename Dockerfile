FROM rustlang/rust:nightly as build

WORKDIR /usr/src

COPY . .

RUN cargo build --release

CMD ["/usr/src/target/release/gaze"]

FROM alpine
COPY --from=build /usr/src/target/release/gaze /
USER root

ENV RUST_LOG=info
ENV RUST_BACKTRACE=full
EXPOSE 6142

CMD ["/gaze"]
