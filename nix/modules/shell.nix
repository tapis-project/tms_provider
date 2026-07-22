{ ... }:
{
  perSystem =
    { inputs', config, ... }:
    {
      devShells.default = (config.rust.craneLib.devShell.override {
        mkShell = inputs'.shell-utils.lib.shell;
      }) {
        name = "TMS-Provider-Dev";
        extraInitRc = ''
          alias sudo='\sudo env PATH="$PATH" HOME="$HOME"'
        '';
        inputsFrom = with config.packages; [
          tms-provider
          wrapped-tms-provider
        ];
        packages = [ ];
      };
    };
}
