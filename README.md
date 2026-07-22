# TMS Resources Provider

This service returns a list of resources available for the provider.

## Running the application

For the out-of-the-box application, run:
```bash
nix run github:tapis-project/tms_provider
```

For running the application from the source code, run:
```bash
nix run
```
in local clone of this repository.

Alternatively, build the application and run it with:
```bash
nix build
./result/bin/tms-provider
```

## Configuring the application

Access the full list of options to configure by running:
```bash
nix run .#docs-serve
```
and change the values of the options in the file `nix/config.nix`.

## Adding a list of resources

*To be written*

## Adding a new data source for resources

*To be written*