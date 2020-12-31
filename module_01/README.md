# MÓDULO 01 - HELLO WORLD

Rode o comando 
```rust
    cargo build --verbose
```

Você verá a seguinte saída:
```rust 
   Compiling module_01 v0.1.0 (/home/rodrigo/Projects/rust/learning/module_01)
     Running `rustc --crate-name module_01 --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=8e1f0fec5ddb9ffe -C extra-filename=-8e1f0fec5ddb9ffe --out-dir /home/rodrigo/Projects/rust/learning/module_01/target/debug/deps -C incremental=/home/rodrigo/Projects/rust/learni
```
Internamente o **cargo** chama o compilador **rustc** e gera o executável do projeto, também gera o arquivo **Cargo.lock** que é apenas um controle interno do rust, e não deve ser alterado pelo programador.