{ self, flake-parts-lib, ... }:
{
  options.perSystem = flake-parts-lib.mkPerSystemOption
    ({ lib, config, pkgs, ... }:
      let
        rustc_version = builtins.readFile (pkgs.runCommand "rustc-version" { } ''
          ${config.rust.binary}/bin/rustc --version > $out
        '');
      in
      {
        options = {
          tms-provider = {
            git_url = lib.mkOption {
              type = lib.types.str;
              default = "https://github.com/tapis-project/tms_provider";
              description = "URL for the remote Git repository";
            };
            version = lib.mkOption {
              type = lib.types.str;
              default = "0.0.0";
              description = "Version of the application";
            };
            git_commit = lib.mkOption {
              type = lib.types.str;
              default = self.shortRev or self.dirtyShortRev or "unknown";
              defaultText = "Short hash or 'unknown'";
              readOnly = true;
              description = "Short hash of the commit that produces the build";
            };
            rust_version = lib.mkOption {
              type = lib.types.str;
              default = "${rustc_version}";
              defaultText = "version of Rust in the current toolchain";
              readOnly = true;
              description = ''
                Version of Rust in toolchain. 
              
                For changing the toolchain, please, use configure `rust` module.
              '';
            };
            RUST_LOG = lib.mkOption {
              type = lib.types.str;
              default = "";
              description = ''
                Value for RUST_LOG.

                Consult https://docs.rs/env_logger/latest/env_logger/#enabling-logging
                for the grammar of this value.
              '';
            };
            address = lib.mkOption {
              type = lib.types.str;
              default = "0.0.0.0";
              description = "Address where TMS Resources Provider will listen";
            };
            port = lib.mkOption {
              type = lib.types.port;
              default = 9000;
              description = "Port where TMS Resources Provider will listen";
            };
            source_kind = lib.mkOption {
              type = lib.types.enum [ "Null" "File" "Database" ];
              default = "File";
              description = ''
                Data source where to obtain the resources.

                - `Null`: a source that returns an empty collection of resources
                - `File`: use a file as source (see an example: [sources-sample.yaml](${config.tms-provider.git_url}/blob/main/assets/sources-sample.yaml))
                - `Database`: not implemented yet
              '';
            };
            source_location = lib.mkOption {
              type = lib.types.nullOr lib.types.str;
              default = null;
              description = ''
                Location of the data source. 
                
                For example, a path for the `File` source,
                or a connection string for the `Database` source.
                
                The default `null` refers to the data file bundled with the source code.
              '';
            };
            jwt_issuers = lib.mkOption {
              type = lib.types.listOf lib.types.str;
              default = [ ];
              description = ''
                List of URLs of issuers accepted for the JWT tokens.

                The URLs must be configured to respond to the standard 
                `<url>/.well-known/openid-configuration`. Do not use trailing slashes.
              '';
            };
            jwt_key_cache_ttl = lib.mkOption {
              type = lib.types.ints.positive;
              default = 300;
              description = ''
                Time to live for the cache that contains the public keys from the
                issuers (in seconds).
              '';
            };
            silent = lib.mkOption {
              type = lib.types.bool;
              default = false;
              description = ''
                Whether to display a banner to stdout at start-up time, 
                with information about configured options.

                Note that with `silent = false`, the information is still available in the 
                logs (for example, setting `RUST_LOG = debug`).
              '';
            };
          };
        };
      });
  config.perSystem = { lib, config, pkgs, ... }:
    let
      tms-provider =
        let
          src = config.rust.craneLib.cleanCargoSource (config.rust.craneLib.path ./../..);
          commonArgs = {
            inherit src;
            buildInputs = with pkgs; [
              pkg-config
              sqlx-cli
              openssl
              git
            ] ++ lib.optionals pkgs.stdenv.isDarwin [ pkgs.libiconv ];
          };
          cargoArtifacts = config.rust.craneLib.buildDepsOnly commonArgs;
        in
        config.rust.craneLib.buildPackage (commonArgs //
          {
            inherit cargoArtifacts;
            meta = {
              description = "TMS Provider";
              mainProgram = "tms_provider";
            };
          });
      wrapped-tms-provider =
        let
          source_location =
            let
              loc = config.tms-provider.source_location;
            in
            if builtins.isNull loc then
              "${./../../assets/data/resources.yaml}"
            else
              loc;
          conf = builtins.toJSON
            {
              inherit source_location;
              inherit (config.tms-provider)
                address
                port
                source_kind
                jwt_issuers
                jwt_key_cache_ttl
                silent;
            };
        in
        pkgs.stdenv.mkDerivation {
          name = "tms-provider";
          nativeBuildInputs = [ pkgs.makeWrapper ];
          buildInputs = [ pkgs.rsync config.shell-utils.tomlMap ];
          dontUnpack = true;
          installPhase = ''
            mkdir -p $out/{bin,etc}
            echo '${conf}' > $out/etc/config.json
            cp ${./../../assets/data/resources.yaml} $out/etc/resources.yaml
            makeWrapper ${lib.getExe tms-provider} $out/bin/tms-provider \
              --set TMS_PROVIDER_VERSION "${config.tms-provider.version}" \
              --set TMS_PROVIDER_COMMIT "${config.tms-provider.git_commit}" \
              --set TMS_PROVIDER_RUST_VERSION "${config.tms-provider.rust_version}" \
              --set RUST_LOG "${config.tms-provider.RUST_LOG}" \
              --set TMS_PROVIDER_CONF_FILE "$out/etc/config.json"
          '';
        };
      # TMS + Postgres for local development.
      # Start Postgres with empty db, run `tms-server --install` on a fresh root
      # directory, and start `tms-server`.
      # tms-server-stack-up = pkgs.writeShellApplication {
      #   name = "tms-server-up";
      #   runtimeInputs = with pkgs; [
      #     mktemp
      #   ];
      #   text = ''
      #     command -v sudo >/dev/null 2>&1 || (printf "Need \`sudo\` to run postgres"; exit 1)
      #     tms-server-down () {
      #       sudo ${config.packages.postgres}/bin/postgres-down
      #     }
      #     trap tms-server-down EXIT
      #     sudo ${config.packages.postgres}/bin/postgres-up
      #     echo "Initializing database"
      #     env \
      #       PATH="$PATH" \
      #       TMS_DB_HOST="${config.tms.TMS_DB_HOST}" \
      #       TMS_DB_PORT="${toString config.tms.TMS_DB_PORT}" \
      #       POSTGRES_USER="${config.postgres.POSTGRES_USER}" \
      #       POSTGRES_PASSWORD="${config.postgres.POSTGRES_PASSWORD}" \
      #       TMS_DB_USER="${config.tms.TMS_DB_USER}" \
      #       TMS_DB_USER_PASSWORD="${config.tms.TMS_DB_USER_PASSWORD}" \
      #       TMS_DB_DB_NAME="${config.tms.TMS_DB_DB_NAME}" \
      #       ${initDb}
      #     TEMP=$(mktemp -d)
      #     ${wrapped-tms-server}/bin/tms-server --install --root-dir "$TEMP"
      #     ${pkgs.gum}/bin/gum style \
      #       --foreground 212 --border-foreground 212 --border double \
      #       --align center --width 50 --margin "1 2" --padding "2 4" \
      #       "TMS Server is running in root-dir = $TEMP"
      #     ${wrapped-tms-server}/bin/tms-server --root-dir "$TEMP"
      #   '';
      # };
      # tms-server-stack-down = pkgs.writeShellApplication {
      #   name = "tms-server-down";
      #   text = ''
      #     ${config.packages.postgres}/bin/postgres-down
      #   '';
      # };
      # tms-server-stack = pkgs.symlinkJoin {
      #   name = "tms-server-stack";
      #   paths = [
      #     tms-server-stack-up
      #     tms-server-stack-down
      #   ];
      # };
    in
    {
      config = {
        # apps = {
        #   default = lib.mkForce {
        #     type = "app";
        #     program = "${tms-server-stack-up}/bin/tms-server-up";
        #   };
        # };
        packages = {
          default = lib.mkForce wrapped-tms-provider;
          inherit tms-provider wrapped-tms-provider;
        };
      };
    };
}
