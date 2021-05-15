### Implementation

* **Job controller**
  - a gRPC API server listening for requests regarding ingestion jobs, ingestion results and dataset requests
  - uses Tokio as coroutine runtime
  - implements a basic in-memeory database

* **Client**
  - REST API server that receives requests from a user and forwards them to the job controller via gRPC; uses Warp crate

* **Communication library**
  - Rust library that contains the generated message and RPC definitions used by the previous components
  - generated from protobuffer definitions with Tonic

### Results

I didnt manage to complete the task but it was a good experience.
Only the creation actions are complete. The getters dont actually return anything (to the REST API), but the internal state of the DB can be constantly seen on the debug messages of the job controller. The job consumer is also not implemented (it never deletes from DB). Also the use of Enums is wrong.

### Design choices

I considered the an oportunity to explore Rust's capabilities and the crate/documentation support, to get an idea if I would like to work in this space. So regarding technological choices, there is no other reason for them than that I wanted to see how they work.

- Why gRPC?

I considered that since you use Rust for the backend services it is very likely you also use gRPC for communication between services since it is more I/O efficient than the traditional REST API

- Then why make a REST\* API?

Initially I wanted to make a REST API, then I switched to gRPC but I didnt know how to interact with gRPC from outside the system, so I decided to keep both and make a thin REST layer in front of the job controller so it is easy to interact with it via Postman or curl. It was more work than I anticipated.

- Database
  - Structure:
     - One single hashmap object for the whole DB with three keys - jobs, results and queues
     - The jobs and results are kept as a hashmap with the job ID as key (although I should probably keep arrays of answers for each job)
  - For simplicity and because of time constraints I chose to not keep two different queues on urgent tasks and normal tasks, the urgent tasks are simply added to the front of the queue, so the urgent jobs work as LIFO while the rest are FIFO, but the urget jobs dont have special priority
  - Also for simplicity I chose to implement the priority queue of Jobs with two data structures - a VecDeque and a HashMap, first for keeping the order and second for quick access to the Job structs
  - To ensure atomicity of the DB operations the database object is placed behing a Mutex provided by tokio

\* I know REST is a more generic term, but is easier to say REST vs gRPC then define it otherwise

### Things I wanted to do but were left undone

- A good test, with multiple clients
- Unit tests
- Results returned by the REST service
- Proper error handling along the communication flow
- Docker compose setup
- Remove potentially unnecessary use of clone()
- Better handling of the enum types between the gRPC and DB definitions
- The vectors holding the job order should be sets
- Provide other means of testing than via Postman

### How to test

The task is not complete, but jobs and results can be created.
To download and import the Postman collection and environment from the `postman` directory.
Then run the two components in different shells and use the POST commands in postman to send requests:

```
cd data_clients && cargo run
cd job_controller && cargo run
```

The calls will not receive responses with populated bodies, but the state of the DB can be seen in the controller logs.
