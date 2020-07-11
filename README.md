<p align="center">
<img src="https://raw.githubusercontent.com/ivosequeros/gaze/master/docs/title.png" height="120"></p>
<p align="center"><b>Gaze is a performant Rust-written event sourcing engine with in-memory storage based on Avro encoding</b></p>

---

### 1. Main structure

On this project there are 3 interesting folders. `src` contains the source code of the program, `kubernetes` contains a definition file to deploy it and `benches` contains benchmarking functions used to test the efficacy of several solutions.

Inside the source folder, there are 4 folders, corresponding to the 4 components of this program: `protocol` (network protocol), `codec`  (message codec), `selection` (message selection) and storage (in-memory message storage). There are 4 more components in the root of src: `server` (runs a TCP server that accepts connections), `router` (holds the clients, storage, global selector and registry), `client` (is created for each incoming client) and `connection` (passed around during the connection to access the router and the connection client).

### 2. How to skim through the code

The program starts at [`main.rs`](https://github.com/ivosequeros/gaze.rs/blob/master/src/main.rs). This file executes the server on a Tokio green thread. The second file to look at is [`server.rs`](https://github.com/ivosequeros/gaze.rs/blob/master/src/server.rs). In this file connections are accepted and a client (see [`client.rs`](https://github.com/ivosequeros/gaze.rs/blob/master/src/client.rs)) is created and added to the router (see [`router.rs`](https://github.com/ivosequeros/gaze.rs/blob/master/src/router.rs)). A connection (see [`connection.rs`](https://github.com/ivosequeros/gaze.rs/blob/master/src/connection.rs)) is also created and the connection is handled to `Connection::accept`.

This function then calls `Eater::eat` (see [`eater.rs`](https://github.com/ivosequeros/gaze.rs/blob/master/src/protocol/eater.rs), which takes care of reading all incoming messages, selecting them and storing them using the rest of the modules.

### 3. Directory sturucture

```
.
├── client.rs            # Connection client
├── codec
│   ├── mod.rs
│   ├── numbers.rs       # Contains traits to encode, decode and zig-zag var-int numbers
│   └── registry.rs      # Structure that holds all message schemas
├── connection.rs        # Connection
├── errors.rs            # Contains possible program errors
├── lib.rs
├── main.rs              # Entrypoint
├── protocol
│   ├── command.rs       # Possible network protocol commands
│   ├── eater.rs         # Ingests messages
│   ├── mod.rs
│   ├── reader.rs        # Reads from stream according to network protocol specs
│   ├── time.rs          # Generates a time representation (not used)
│   └── writer.rs        # Writes to stream according to network protocol specs
├── router.rs
├── selection
│   ├── filter.rs        # Represents and takes care of building a message filter
│   ├── mod.rs
│   ├── selector.rs      # Message selector
│   └── subscription.rs  # Client subscription: contains its filter and methods to integrate into a common selector
├── server.rs            # Server
└── storage
    └── mod.rs           # Storage structure and methods
```

### 4. How to run
#### Raw
To run this project, make sure you have [Rust](https://rustup.rs/) installed. Clone the repo and run `cargo run`.

#### With Docker
Make sure you have [Docker](https://docs.docker.com/get-docker/) installed. Run `docker build .` on the root of this project to build a Docker image. Once the image has been build, run `docker run <image-name>`.


### 5. Client
You can find the client library and use examples in the [gaze.rs client repo](https://github.com/ivosequeros/gaze.rs-client).
