<p align="center">
<img src="https://raw.githubusercontent.com/ivosequeros/gaze/master/docs/title.png" height="120"></p>
<p align="center"><b>Gaze is a performant Rust-written event sourcing engine with in-memory storage based on Avro encoding</b></p>

---

### 1. Structure

On this project there are 3 interesting folders. `src` contains the source code of the program, `kubernetes` contains a definition file to deploy it and `benches` contains benchmarking functions used to test the efficacy of several solutions.

Inside the source folder, there are 4 folders, corresponding to the 4 components of this program: `protocol` (network protocol), `codec`  (message codec), `selection` (message selection) and storage (in-memory message storage). There are 4 more components in the root of src: `server` (runs a TCP server that accepts connections), `router` (holds the clients, storage, global selector and registry), `client` (is created for each incoming client) and `connection` (passed around during the connection to access the router and the connection client).


### 2. How to run
#### Raw
To run this project, make sure you have [Rust](https://rustup.rs/) installed. Clone the repo and run `cargo run`.

#### With Docker
Make sure you have [Docker](https://docs.docker.com/get-docker/) installed. Run `docker build .` on the root of this project to build a Docker image. Once the image has been build, run `docker run <image-name>`.
