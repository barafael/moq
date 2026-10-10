# MoQ CI host

The `ci-runner` NixOS module configures ephemeral x86 runners. It does not yet
route workflows onto them: the local cache server needs a stable release with
anonymous reads and OIDC-authorized writes. Keep `CI_RUNNER` unset until that
policy and the workflow integration have been tested.

Create a dedicated GitHub App with organisation **Self-hosted runners: write**,
install it on `moq-dev`, and create an organisation runner group named `moq`
restricted to `moq-dev/moq`. Deploy its PEM key as a root-readable runtime secret.
The NixOS runner module derives short-lived tokens on each registration; the
key never enters the Nix store or the job environment.

```nix
{
  imports = [ moq.nixosModules.ci-runner ];
  services.moq-ci = {
    enable = true;
    appId = 123456;
    privateKeyFile = "/run/secrets/moq-ci.pem";
    warmShell = moq.devShells.x86_64-linux.default;
    count = 4;
    # Enable only after the NVIDIA host driver is configured.
    gpu.enable = false;
  };
}
```

Each runner uses a dynamic user, a fresh work directory, its dedicated label,
and the upstream sandbox defaults. CPU and GPU jobs share the Nix daemon and
store. Enabling GPU access relaxes device isolation for the GPU runner only.
Nix retains the selected shell as a system closure dependency and collects
other paths when disk space falls below 10 GiB, freeing towards 30 GiB.
Runner services allow 64 MiB locked memory for concurrent io_uring tests.

The latest tagged `jdx/mr-boxington-cache` server is v0.1.1. Its anonymous mode
also permits writes. Do not deploy it with anonymous access for this workload.
The next integration must prove that unauthenticated and PR-ref writes fail,
while only `cache.yml` on `main` can write, before enabling workflow routing.
