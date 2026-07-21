{ inputs, flake-parts-lib, ... }:
{
  imports = [
    inputs.flake-parts-website.flakeModules.empty-site
  ];
  options.perSystem = flake-parts-lib.mkPerSystemOption
    ({ lib, pkgs, config, ... }:
      let
        mdBookProject = pkgs.stdenv.mkDerivation {
          name = "mdBook-project";
          buildInputs = [ pkgs.mdbook ];
          src = ./.;
          buildPhase = ''
            mkdir -p $out
            cd $out
            mdbook init --force --title "${config.render.inputs.tms-provider.title}"
            cat <<EOF >src/SUMMARY.md
              # Summary

              - [Options](./options.md)
            EOF
            cp ${config.packages.generated-docs-tms-provider}/options.md src
            rm src/chapter_1.md
            mdbook build
          '';
        };
        serve = pkgs.writeShellApplication {
          name = "docs-serve";
          runtimeInputs = [
            pkgs.python3
            pkgs.coreutils
            pkgs.xdg-utils
            config.shell-utils.findPort
          ];
          text = ''
            cd ${mdBookProject}/book
            PORT="$(find-port)"
            trap 'kill $(jobs -p) && echo "Documentation server stopped"' EXIT
            python3 -m http.server "$PORT" &
            ${pkgs.gum}/bin/gum style \
            --foreground 212 --border-foreground 212 --border double \
            --align center --width 50 --margin "1 2" --padding "2 4" \
            "Documentation available at http://localhost:$PORT"
            xdg-open http://localhost:"$PORT"
            sleep infinity
          '';
        };
      in
      {
        options = {
          documentation = {
            mdBookProject = lib.mkOption {
              type = lib.types.package;
              default = mdBookProject;
            };
            serve = lib.mkOption {
              type = lib.types.package;
              default = serve;
            };
          };
        };
      });
  config = {
    flake.flakeModule = import ./default.nix;
    perSystem = { config, ... }: {
      packages = {
        inherit (config.documentation) mdBookProject;
        docs-serve = config.documentation.serve;
      };
      render.inputs.tms-provider = {
        flake = inputs.self;
        baseUrl = "${config.tms-provider.git_url}/blob/main";
        intro = ''
          Introduction to TMS Provider.
        '';
        installation = ''
          Installation instructions for TMS Provider.
        '';
        title = "TMS Provider";
      };
    };
  };
}
