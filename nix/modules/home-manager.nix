{
  config,
  lib,
  pkgs,
  ...
} @ s: let
  inherit (lib.modules) mkIf;
  inherit (lib.options) mkEnableOption mkOption mkPackageOption;

  cfg = config.services.wayle;
  tomlFormat = pkgs.formats.toml {};
in {
  options.services.wayle = {
    enable = mkEnableOption "wayle shell";
    package = mkPackageOption pkgs "wayle" {nullable = true;};

    systemd.enable =
      (mkEnableOption "wayle systemd service")
      // {
        default = cfg.enable;
        example = false;
      };

    settings = mkOption {
      type = lib.types.attrs;
      description = ''
        Standard configuration options for wayle.
      '';

      default = {};

      example = {
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
      };
    };
  };

  config = mkIf cfg.enable {
    home.packages = mkIf (cfg.package != null) [cfg.package];

    xdg.configFile."wayle/config.toml" = mkIf (cfg.settings != {}) {
      source = tomlFormat.generate "wayle-config" cfg.settings;
    };

    systemd.user.services.wayle = mkIf (cfg.systemd.enable && cfg.package != null) {
      Unit.Description = "Wayle - Shell";
      Install.WantedBy = ["graphical-session.target"];
      Service = {
        ExecStart = "${lib.getExe' cfg.package "wayle-shell"}";
        Restart = "on-failure";
      };
    };
  };
}
