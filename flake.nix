{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix.url = "github:nix-community/fenix";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      fenix,
      utils,
      ...
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            (pkgs.fenix.complete.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
            ])

            pkgs.rust-analyzer-nightly
          ];

          RUST_SRC_PATH = "${pkgs.fenix.complete.rust-src}/lib/rustlib/src/rust/library";
        };

        packages.default =
          let
            manifest = pkgs.lib.importTOML ./Cargo.toml;
          in
          (pkgs.makeRustPlatform {
            cargo = pkgs.fenix.minimal.toolchain;
            rustc = pkgs.fenix.minimal.toolchain;
          }).buildRustPackage
            {
              pname = manifest.package.name;
              inherit (manifest.package) version;
              src = pkgs.lib.cleanSource ./.;
              cargoLock.lockFile = ./Cargo.lock;
              meta.mainProgram = manifest.package.name;
            };
      }
    )
    // {
      nixosModules.default =
        {
          config,
          lib,
          pkgs,
          ...
        }:
        let
          cfg = config.services.streamed-88x31;
        in
        {
          options.services.streamed-88x31 = {
            enable = lib.mkEnableOption "Streamed 88x31 button";

            package = lib.mkOption {
              type = lib.types.package;
              default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
            };

            host = lib.mkOption {
              type = lib.types.nullOr lib.types.str;
              default = null;
            };

            port = lib.mkOption { type = lib.types.port; };
          };

          config =
            let
              port = toString cfg.port;
            in
            lib.mkIf cfg.enable {
              systemd.services.streamed-88x31 = {
                serviceConfig = {
                  CapabilityBoundingSet = "";
                  DynamicUser = true;

                  ExecStart = "${lib.getExe cfg.package} ${
                    lib.escapeShellArgs (
                      lib.optionals (cfg.host != null) [
                        "--host"
                        cfg.host
                      ]
                      ++ lib.optionals (cfg.port != null) [
                        "--port"
                        port
                      ]
                    )
                  }";

                  KillSignal = "SIGINT";
                  LockPersonality = true;
                  MemoryDenyWriteExecute = true;
                  PrivateDevices = true;
                  PrivateIPC = true;
                  PrivatePIDs = true;
                  ProcSubset = "pid";
                  ProtectClock = true;
                  ProtectControlGroups = "strict";
                  ProtectHome = true;
                  ProtectHostname = true;
                  ProtectKernelLogs = true;
                  ProtectKernelModules = true;
                  ProtectKernelTunables = true;
                  ProtectProc = "noaccess";
                  Restart = "on-failure";

                  RestrictAddressFamilies = [
                    "AF_INET"
                    "AF_INET6"
                  ];

                  RestrictNamespaces = true;
                  RestrictRealtime = true;
                  SocketBindAllow = port;
                  SystemCallArchitectures = "native";
                  SystemCallErrorNumber = "EPERM";
                  SystemCallFilter = [ "@system-service" ];
                };

                wantedBy = [ "multi-user.target" ];
              };
            };
        };
    };
}
