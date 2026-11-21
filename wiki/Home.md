# SigmaOS-airbyte

SigmaOS integration component for **Airbyte data integration**.

## Commands

```bash
sigmaos-airbyte install   # install package + deploy defaults
sigmaos-airbyte deploy    # copy config templates to $HOME
sigmaos-airbyte status    # binary/package/config report
sigmaos-airbyte reset     # remove deployed config
sigmaos-airbyte open      # launch the application
```

## Design

- Shell-based component manager (Omarchy-style per-app repos)
- Secure defaults; no secrets stored
- Deterministic `test.sh` with isolated `$HOME`
