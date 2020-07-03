FROM rustlang/rust:nightly as build

WORKDIR /usr/src

COPY . .


RUN cargo build --release

EXPOSE 6142

CMD ["/usr/src/target/release/gaze"]
