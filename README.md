# WebAssembly com Rust

Este projeto demonstra como criar e executar código WebAssembly (Wasm) usando Rust. Abaixo estão os passos e soluções para problemas comuns encontrados durante o desenvolvimento.

## Configuração e Compilação

- Instale o wasm-pack:

```bash
cargo install wasm-pack
```

```bash
cargo new wasm_hello_world --lib
```

- No arquivo Cargo.toml, adicione as seguintes dependências:

```bash
[dependencies]
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["console"] }

```

```bash
wasm-pack build --target web
```
```html
<script type="module">
    import init, { greet, add } from './pkg/wasm_hello_world.js';

    async function runWasm() {
        await init();
        greet();
        console.log(add(5, 7));
    }

    runWasm();
</script>
```


- Use um servidor local para servir sua página e evitar problemas de CORS. Uma opção é o http-server com Node.js ou o módulo http.server com Python.

## Problemas Comuns e Soluções

- Se encontrar erros relacionados a versões do Rust ou pacotes, garanta que você está usando versões compatíveis de `rustc`, `cargo`, `wasm-pack` e outras ferramentas.

- Se o console mostrar erros de tipo MIME ao carregar o arquivo `.wasm`, verifique se o servidor está configurado para servir arquivos `.wasm` com o tipo MIME `application/wasm`.

- Se o arquivo `.wasm` estiver pendente (em estado "pending") no painel de rede do navegador, verifique a configuração do servidor, o caminho do arquivo no código JS e possíveis problemas de CORS.
