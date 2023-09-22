# rustfri

## Technologies used
* Rust
* Cargo
* Docker

### Prerequisites
Make sure you have the rust installed using this command:
#### Rust
```bash script
rustc --version
```

#### Cargo
Make sure you have cargo installed using this command:
```bash script
cargo --version
```

#### Docker
Make sure you have cargo installed using this command:
```bash script
docker --version
```

### Build
Build the code without running it
```bash script
cargo build
```

### Run
Run the code
```bash script
cargo run
```

#####  Create docker image of app
Creating a docker image should be as simple as
``` bash
docker build -t rustfri .
```

#### Running a docker image
``` bash
docker run --rm -it -p 8080:8080 rustfri
```

##### 🧪 Testing the applications

Request to root
```bash script
curl --location --request GET 'http://0.0.0.0:8080'
```

