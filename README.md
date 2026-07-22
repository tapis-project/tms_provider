# TMS Resources Provider

This service returns a list of resources available for the provider.

## Running the application

For the out-of-the-box application, run:
```bash
nix run github:tapis-project/tms_provider
```

For running the application from the source code, clone this repository and run:
```bash
nix run
```

Alternatively, build the application and run it with:
```bash
nix build
./result/bin/tms-provider
```

## Accessing the service

The out-of-the-box application listens in port 9000 and authenticates with tokens
from `https://dev.develop.tapis.io/v3/tokens`. For an easy experience, run the development
environment which provides `httpie` and a function to obtain a token:
```bash
nix develop
TOKEN=$(get-token)
http localhost:9000/resources Authorization:"Bearer $TOKEN"
```

## Configuring the application

Access the full list of options to configure by running:
```bash
nix run .#docs-serve
```
in the local clone of the repository,
and change the values of the options in the file `nix/config.nix`. After changing
the config, ensure you run `nix build`.

## Adding a list of resources

*To be written*

## Adding a new data source for resources

*To be written*