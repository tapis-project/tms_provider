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
          tms = {
            git_commit_short = lib.mkOption {
              type = lib.types.str;
              default = self.shortRev or self.dirtyShortRev or "unknown";
              defaultText = "Short hash or 'unknown'";
              readOnly = true;
              description = "Short hash of the commit that produces the build";
            };
            rustc_version = lib.mkOption {
              type = lib.types.str;
              default = "${rustc_version}";
              defaultText = "version of Rust in the current toolchain";
              readOnly = true;
              description = ''
                Version of Rust in toolchain. 
              
                For changing the toolchain, please, use configure `rust` module.
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
            GIT_COMMIT_SHORT = config.tms.git_commit_short;
            RUSTC_VERSION = config.tms.rustc_version;
            meta = {
              description = "TMS Provider";
              mainProgram = "tms_provider";
            };
          });
      # TMS Server that bundles the `resources` directory, so it can install
      # the config files without needing the source files.
      # wrapped-tms-server = pkgs.stdenv.mkDerivation {
      #   name = "tms-server";
      #   src = ./../../resources;
      #   nativeBuildInputs = [ pkgs.makeWrapper ];
      #   buildInputs = [ pkgs.rsync config.shell-utils.tomlMap ];
      #   installPhase = ''
      #     mkdir -p $out/{bin,src/resources}
      #     rsync -a . $out/src/resources/
      #     cat config/tms.toml | toml-map '
      #     doc["http_addr"] = "${config.tms.TMS_SERVER_HOST}"
      #     doc["http_port"] = ${toString config.tms.TMS_SERVER_PORT}
      #     ' > $out/src/resources/config/tms.toml
      #     makeWrapper ${lib.getExe tms-server} $out/bin/tms-server \
      #       --set TMS_RESOURCES_DIR $out/src/resources \
      #       --set TMS_ROOT_DIR ${config.tms.TMS_ROOT_DIR} \
      #       --set TMS_DB_HOST ${config.tms.TMS_DB_HOST} \
      #       --set TMS_DB_PORT ${toString config.tms.TMS_DB_PORT} \
      #       --set TMS_DB_USER ${config.tms.TMS_DB_USER} \
      #       --set TMS_DB_USER_PASSWORD ${config.tms.TMS_DB_USER_PASSWORD} \
      #       --set TMS_DB_DB_NAME ${config.tms.TMS_DB_DB_NAME}
      #   '';
      # };
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
          default = lib.mkForce tms-provider;
          inherit tms-provider;
        };
      };
    };
}
