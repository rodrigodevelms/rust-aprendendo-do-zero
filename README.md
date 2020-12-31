# APRENDENDO RUST DO ZERO

<br>

### MENU

* PRÉ REQUESITOS


<br>

---

#### PRÉ REQUESITOS

 - AMBIENTE LINUX: Antes de instalar o compilador RUST você precisa ter algumas ferramentas pré-instaladas para poder fazer a compilação.
    
   - Em distribuições **Debian** e derivados faça: 
   ` sudo apt install build-essential `
   - Na distribuição **Solus** faça: 
   ` sudo eopkg it -c system.devel `
   - Em outras distribuições, consulte a **documentação**.
   - Mesmo já instalando o **build-essentials** ainda ocorrer o erro:   ` linker cc not found `  instale o CMAKE.

<br>

----

#### INSTALAÇÃO

Você pode instalar o compilador **RUST** de algumas maneiras, mas o recomendável é através do **RUSTUP**. Rustup nada mais é que um instalador da linguagem, contendo o compilador, o Cargo (mais detalhes abaixo). Acesse https://rustup.rs/ para instalar.

<br>

----

#### RUSTC E CARGO

**rustc** é o compilador da linguagem. Ele irá compilar o seu código produzindo um binário, uma biblioteca e um executável. Geralmente os programadores rust não acessam diretamente o rustc, ao invés disso usam o **cargo**. O Cargo é um gerenciador de pacotes, mas vai além disso. Ele também compila o seu código, chamando o rustc por debaixo dos panos. 

<br>

---

### INTRODUÇÃO

Esse estudo será separado por módulos, a cada novo módulo criaremos um projeto com 
```rust 
 cargo new nome_do_projeto
 ```
exemplo: 
```rust 
 cargo new module_01 
 ```
