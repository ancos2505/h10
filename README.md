# h10

Simple HTTP/1.0 Server with no external dependencies (WIP)

## Getting Started


### Installation
```
cargo install h10
```

### Installed mode
```
./h10-server --http1.0 --ip-address=127.0.0.1 --port=9000 
```
or 
```
./h10-server --help
```

### Dev mode

#### Terminal 1
```
cargo run
```
#### Terminal 2
```
curl -v localhost:8080
```

or open in you browser: http://localhost:8080/

## Roadmap

- Serve static files sandboxed in a specific folder
- Upload files
- Implement the complete spec [RFC1945](https://www.rfc-editor.org/rfc/rfc1945.html)
