# Verifier Client

The `verifier-client` is a Rust-based application designed to interact with the transparency log service. It is responsible for requesting inclusion proofs for a given artifact, a given commit hash, and the received attestation document.

## Preparations

To install the `verifier-client`, you need to have Rust and Cargo installed on your system. You can install Rust and Cargo by following the instructions on the [official Rust website](https://www.rust-lang.org/).

Clone the repository and navigate to the project directory:

```shell
cd verifier-client
cargo build
```

Prepare your .env file with the following content:

```yaml
# Example .env file
VERIFIER_PERSONALITY_BASE_URL=http://localhost:8090
VERIFIER_TREE_SIZE=42
VERIFIER_LOG_ID=9238749836740
```

Environment variables:

- VERIFIER_PERSONALITY_BASE_URL: The base url of the personality
- VERIFIER_TREE_SIZE: The current tree size you want to request an inclusion proof
- VERIFIER_LOG_ID: The actual log id. You can request available log ids via the personality `/list-trees`.

## Run it locally

It is possible to run the verifier-client in local mode and some switches to simulate other calls:

```shell
cargo run --bin verifier-client --commit-hash "commit-hash" --artifact-hash "artifact-hash" --artifact-name "artifact-name" --attestation-document "attestation-document"
```
