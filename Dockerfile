FROM rustlang/rust:nightly

WORKDIR /usr/src

COPY . .

RUN cargo build --release
EXPOSE 6142

CMD ["/usr/src/target/release/gaze"]