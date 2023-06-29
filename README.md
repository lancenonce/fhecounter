# 🔄👾🧮
## The FHE Counter
This is a simple server-client counter that uses the [tfhe-rs](https://github.com/zama-ai/tfhe-rs) library to increment an encrypted number with another encrypted value on a server. I learned a lot doing this and hope to understand the mathematics behind lattices and fully homomorphic encryption better (if you're an expert, would love to chat).

**Warning:** This project is still under production and not quite ready to use. Please see the Issues tab for current road blocks. 
## Installation and Usage
1: Clone the repo
2: Build the binaries with (please use release, FHE is computationally intense): 
```bash
cargo build --release
```
3: Run the server. I use port 8080 here, but just make sure the client and server are using the same port. This project is not designed for multiple machine usage.
```bash
# Pass in the port number as an argument
cargo run --release server 8080
```
4: Run the client:
```bash
# Pass in the port number and an initiel value as an argument
cargo run --release client 8080 38
```
5: Pass in commands. 
  - On the client's CLI, type "increment" to increase your initial value. This will prompt the CLI to ask your step number (all good as long as it's <5)
  - When you want to receive and decrypt your number, type "get"

## Contributing
Please feel free to contribute to this small project. It's great for FHE beginners such as myself. Here are some resources I recommend for going deeper into the ma'at: 
- [Intro to lattice based crypto and Homomorphic encryption](https://arxiv.org/pdf/2208.08125.pdf)
- [Zama's Blog](https://www.zama.ai/blog)
