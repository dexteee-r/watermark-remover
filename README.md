# Watermark Remover

Un petit utilitaire Rust qui lit un texte généré par un LLM, supprime le watermark, et produit un fichier net .

## Build & Run
```bash
rtk cargo build --release
rtk cargo run --quiet --release
```

## Tests
```bash
rtk cargo test
```

## CI
Workflow GitHub Actions est configuré dans `.github/workflows/ci.yml`.
