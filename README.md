# Airbyte

Airbyte is a SigmaOS component providing a functional domain model for `airbyte2` with validated state, deterministic storage, and typed errors.

## Description

Airbyte data integration compatibility layer for SigmaOS

## Domain model

- `DataIntegration` — domain entity

## API

- `new()`

## Testing

```sh
cargo test
```

Tests are deterministic, isolated, and self-validating.
