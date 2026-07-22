{ ... }:
{
  perSystem =
    { inputs', pkgs, config, ... }:
    {
      devShells.default = (config.rust.craneLib.devShell.override {
        mkShell = inputs'.shell-utils.lib.shell;
      }) {
        name = "TMS-Provider-Dev";
        extraInitRc = ''
          alias sudo='\sudo env PATH="$PATH" HOME="$HOME"'
          alias get-token='http --check-status https://dev.develop.tapis.io/v3/oauth2/tokens \
            Content-type:application/json username=testuser2 password=testuser2 grant_type=password \
            | jq -r .result.access_token.access_token'
        '';
        inputsFrom = with config.packages; [
          tms-provider
          wrapped-tms-provider
        ];
        packages = [ pkgs.httpie pkgs.jq ];
      };
    };
}
