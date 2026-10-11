{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.moq-ci;
  runner = label: {
    enable = true;
    url = "https://github.com/moq-dev";
    githubApp = {
      id = cfg.appId;
      login = "moq-dev";
      privateKeyFile = cfg.privateKeyFile;
    };
    runnerGroup = cfg.runnerGroup;
    ephemeral = true;
    noDefaultLabels = true;
    extraLabels = [ label ];
    extraPackages = with pkgs; [
      git
      nix
    ];
    serviceOverrides.LimitMEMLOCK = "64M";
  };
in
{
  options.services.moq-ci = {
    enable = lib.mkEnableOption "ephemeral MoQ CI runners";
    count = lib.mkOption {
      type = lib.types.ints.positive;
      default = 4;
      description = "Number of independent moq-ci runners.";
    };
    appId = lib.mkOption {
      type = lib.types.ints.positive;
      description = "ID of the dedicated organisation runner GitHub App.";
    };
    privateKeyFile = lib.mkOption {
      type = lib.types.addCheck (lib.types.strMatching "^/.*") (
        path: !(lib.hasPrefix "${builtins.storeDir}/" path) && path != builtins.storeDir
      );
      description = "Absolute runtime string path to the App key, outside the Nix store.";
    };
    runnerGroup = lib.mkOption {
      type = lib.types.str;
      default = "moq";
      description = "Organisation runner group restricted to moq-dev/moq.";
    };
    warmShell = lib.mkOption {
      type = lib.types.package;
      description = "Current development shell closure retained as a GC root.";
    };
    gpu.enable = lib.mkEnableOption "a separate moq-gpu runner";
  };
  config = lib.mkIf cfg.enable {
    assertions = [
      {
        assertion = pkgs.stdenv.hostPlatform.system == "x86_64-linux";
        message = "MoQ CI runners require x86_64-linux.";
      }
    ];
    services.github-runners = {
      moq-ci = (runner "moq-ci") // {
        count = cfg.count;
      };
    }
    // lib.optionalAttrs cfg.gpu.enable {
      moq-gpu = (runner "moq-gpu") // {
        serviceOverrides = {
          LimitMEMLOCK = "64M";
          PrivateDevices = false;
          DevicePolicy = "closed";
          DeviceAllow = [ "char-nvidia* rw" ];
        };
      };
    };
    nix.settings = {
      substituters = [ "https://kixelated.cachix.org" ];
      trusted-public-keys = [ "kixelated.cachix.org-1:CmFcV0lyM6KuVM2m9mih0q4SrAa0XyCsiM7GHrz3KKk=" ];
      min-free = 10 * 1024 * 1024 * 1024;
      max-free = 30 * 1024 * 1024 * 1024;
    };
    nix.gc.automatic = true;
    environment.etc."moq-ci/dev-shell".source = cfg.warmShell;
  };
}
