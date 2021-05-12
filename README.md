Implementation structure:

* *Job controller* - a gRPC API server listening for requests regarding ingestion jobs, ingestion results and dataset requests
* *Client* - REST API server that receives requests from a user and forwards them to the job controler via gRPC
* *job library* - Rust library that contains the generated message and RPC definitions used by the the previous components


