<p align="center">
<img src="https://raw.githubusercontent.com/ivosequeros/gaze/master/docs/title.png" height="150"></p>
<p align="center"><strong>Gaze is a performant Rust-written event sourcing engine with in-memory storage based on Avro encoding</strong></p>

---

### 1. Structure

On this project there are 3 interesting folders. `src` contains the source code of the program, `kubernetes` contains a definition file to deploy it and `benches` contains benchmarking functions used to test the efficacy of several solutions.


### 2. How to run
#### Raw
To run this project, make sure you have[Rust](h ttps://rustup.rs/) installed. Clone the repo and run `cargo run`.

#### With Docker
Make sure you have [Docker](https://docs.docker.com/get-docker/) installed. Run `docker build .` on the root of this project to build a Docker image. Once the image has been build, run `docker run <image-name>`.
