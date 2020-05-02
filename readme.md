## Nex 

Nex (pronounced as next) is a thing written in Rust and inspired by folding home. It has server which host jobs and clients which connects with server through GPRC and asks for job then they run jobs and upload results back to server. Jobs are run inside WASM VM which means jobs cannot escape its isolation.