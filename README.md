# rust-aws-cloudformation
  Rust application binary to manipulate AWS Clouformation SDK.  

  Developed by: <a href="https://www.github.com/gabrielAlonsoCabral">@GabrielAlonsoCabral</a>  
 <br/>

## Installation

```
# clone this repository
$ git clone https://github.com/GabrielAlonsoCabral/rust-aws-cloudformation.git
$ cd rust-aws-cloudformation
```

<br/>


## Usage

```
# Create a .env file with all his Cloudformation Outputs
$ cargo run --bin create-env

# Find for a variable in Cloudformation Outputs
$ cargo run --bin find-env --  --stage StageToFind --name NameToFind
```


## Build

```
$ cargo build --release
```

## Running binary application

```
# Create a .env file with all his Cloudformation Outputs
$ ./target/release/create-env

# Find for a variable in Cloudformation Outputs
$ ./target/release/find-env --stage StageToFind --name NameToFind
```
