{
  config,
  lib,
  pkgs,
  ...
}:
let
  inherit (builtins) elem;
  inherit (lib) lists;
  inherit (lib.attrsets) attrByPath hasAttrByPath;
  inherit (lib.modules) mkIf;
  inherit (lib.options) mkEnableOption mkOption mkPackageOption;

  cfg = config.services.wayle;

  tomlFormat = pkgs.formats.toml { };
in
{
  meta.maintainers = with lib.maintainers; [ isaacST08 ];

  options.services.wayle = {
    enable = mkEnableOption "wayle shell";
    package = mkPackageOption pkgs "wayle" { nullable = true; };

    systemd.enable = (mkEnableOption "wayle systemd service") // {
      default = true;
      example = false;
    };

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

  config = mkIf cfg.enable {
    assertions = [
      (lib.hm.assertions.assertPlatform "services.wayle" pkgs lib.platforms.linux)
    ];

    home.packages = (
      (lib.lists.optional (cfg.package != null) cfg.package)
      # Install the appropriate theme-provider, if set.
      ++ (
        let
          themeProvider = attrByPath [ "styling" "theme-provider" ] "wayle" cfg.settings;
        in
        (lists.optional (elem themeProvider [
          "matugen"
          "wallust"
          "pywal"
        ]) pkgs.${themeProvider})
      )
    );

    # Main config file.
    xdg.configFile."wayle/config.toml" = mkIf (cfg.settings != { }) {
      source = tomlFormat.generate "wayle-config" cfg.settings;
      force = true; # Wayle aggressively produces its own config file.
    };

    systemd.user.services = mkIf (cfg.systemd.enable && cfg.package != null) {
      # Systemd service for main wayle shell.
      wayle = {
        Unit.Description = "Wayle - Shell";
        Install.WantedBy = [ "graphical-session.target" ];
        Service = {
          ExecStart = "${lib.getExe' cfg.package "wayle-shell"}";
          Restart = "on-failure";
        };
      };

      # Systemd service to run the wallpaper cycling.
      wayle-wallpaper =
        mkIf
          (
            attrByPath [ "wallpaper" "cycling-enabled" ] false cfg.settings
            && "" != attrByPath [ "wallpaper" "cycling-directory" ] "" cfg.settings
          )
          {
            Unit.Description = "Wayle - Wallpaper Cycling";
            Install.WantedBy = [ "graphical-session.target" ];
            Service = {
              ExecStart =
                let
                  optionalFlag =
                    settingPath: fn:
                    if (hasAttrByPath settingPath cfg.settings) then
                      fn (attrByPath settingPath null cfg.settings)
                    else
                      "";
                in
                ''
                  ${lib.getExe' cfg.package "wayle"} wallpaper cycle \
                    ${optionalFlag [ "wallpaper" "cycling-interval-mins" ] (x: "-i ${toString (x * 60)}")} \
                    ${optionalFlag [ "wallpaper" "cycling-mode" ] (x: "-m ${x}")} \
                    "${cfg.settings.wallpaper.cycling-directory}"
                '';
              Restart = "on-failure";
            };
          };
    };

    # Wallpaper-engine dependency.
    services.swww.enable = mkIf (attrByPath [ "wallpaper" "engine-enabled" ] false cfg.settings) (
      lib.mkDefault true
    );
  };
}
