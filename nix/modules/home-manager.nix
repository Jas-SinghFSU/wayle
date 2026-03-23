{
  config,
  lib,
  pkgs,
  ...
}:
let
  inherit (builtins) elem;
  inherit (lib.attrsets) recursiveUpdate;
  inherit (lib) lists mkDefault getExe';
  inherit (lib.modules) mkIf;
  inherit (lib.options) mkEnableOption mkOption mkPackageOption;

  cfg = config.services.wayle;

  tomlFormat = pkgs.formats.toml { };
in
{
  meta.maintainers = with lib.maintainers; [ isaacST08 ];

  options.services.wayle = {
    enable = mkEnableOption "wayle shell";
    package = mkPackageOption pkgs "wayle" { };

    settings = mkOption {
      type = tomlFormat.type;
      description = ''
        Standard configuration options for wayle.
      '';

      default = { };

      example =
        lib.literalExpression
          # nix
          ''
            styling = {
              theme-provider = "wayle";

              palette = {
                bg = "#16161e";
                fg = "#c0caf5";
                primary = "#7aa2f7";
              };
            };

            bar = {
              scale = 1;
              location = "top";
              rounding = "sm";

              layout = {
                monitor = "*";
                left = ["clock"];
                center = ["media"];
                right = ["battery"];
              };
            };

            modules.clock = {
              format = "%H:%M";
              icon-show = true;
              label-show = true;
            };
          '';
    };
  };

  config = mkIf cfg.enable (
    let
      # Define default settings.
      settings = recursiveUpdate {
        wallpaper.engine-enabled = false;
        styling = {
          theme-provider = "wayle";
          wallust-apply-globally = true;
          pywal-apply-globally = true;
        };
      } cfg.settings;
    in
    {
      assertions = [
        (lib.hm.assertions.assertPlatform "services.wayle" pkgs lib.platforms.linux)
      ];

      home.packages = (
        [ cfg.package ]
        # Alias awww to swww.
        ++ (lists.optional settings.wallpaper.engine-enabled (
          pkgs.writeShellScriptBin "awww" ''
            exec swww "$@"
          ''
        ))
        # Install the appropriate theme-provider, if set.
        ++ (lists.optional (elem settings.styling.theme-provider [
          "matugen"
          "wallust"
          "pywal"
        ]) pkgs.${settings.styling.theme-provider})
      );

      # Main config file.
      xdg.configFile."wayle/config.toml" = mkIf (cfg.settings != { }) {
        source = tomlFormat.generate "wayle-config" cfg.settings;
        force = mkDefault true; # Wayle aggressively produces its own config file.
      };

      # Systemd service for main wayle shell.
      systemd.user.services.wayle = {
        Unit.Description = "Wayle - Shell";
        Install.WantedBy = [ "graphical-session.target" ];
        Service = {
          ExecStart = "${getExe' cfg.package "wayle-shell"}";
          Restart = "on-failure";
        };
      };

      # Wallpaper-engine dependency.
      services.swww.enable = mkIf settings.wallpaper.engine-enabled (lib.mkDefault true);

      # If wallust or pywal is the theme-provider and is enabled globally,
      # ensure the theme gets sourced for new terminals.
      programs.bash.bashrcExtra =
        with settings.styling;
        let
          sequenceFile = "${config.xdg.cacheHome}/${
            if (theme-provider == "wallust") then "wallust" else "wal"
          }/sequences";
        in
        mkIf
          (
            elem theme-provider [
              "wallust"
              "pywal"
            ]
            && settings.styling."${theme-provider}-apply-globally"
          )
          # bash
          ''
            [[ -f ${sequenceFile} ]] && ${getExe' pkgs.coreutils "cat"} ${sequenceFile}
          '';
    }
  );
}
