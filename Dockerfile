FROM rustlang/rust:nightly as build

WORKDIR /usr/src

COPY . .

ENV RUST_LOG=info
ENV RUST_BACKTRACE=full

RUN cargo build --release

EXPOSE 6142

CMD ["/usr/src/target/release/gaze"]
