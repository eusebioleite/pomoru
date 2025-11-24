# pomoru

Um pomodoro **extremamente simples** para o terminal. Sem distrações, sem interfaces gigantescas, sem dependências externas além do próprio binário. Execute e ele começa.

## 🕒 O que é o pomoru?
O pomoru é um timer pomodoro minimalista. Quando você executa o binário:

- Ele inicia imediatamente uma sessão de **work**.
- Quando acaba, alterna automaticamente para **rest**.
- Depois volta para work.
- E segue alternando **infinitamente**, até você encerrar com `CTRL + C`.

Só isso.

---

## 🚀 Como usar
### Iniciar o pomodoro
```
pomoru
```
Isso começa imediatamente o ciclo **work → rest → work → rest...**

### Acessar ajuda
```
pomoru h
```

### Ajustar duração de work
```
pomoru w <minutos>
```
Exemplo:
```
pomoru w 30
```

### Ajustar duração de rest
```
pomoru r <minutos>
```
Exemplo:
```
pomoru r 10
```

---

## ⚙️ Arquivo de configuração
O pomoru mantém um arquivo `pomoru.toml` **na mesma pasta do binário**.

Se o arquivo não existir, ele será criado automaticamente com:
```toml
work = 25
rest = 5
```

Modificar configurações via CLI atualiza esse arquivo.

---

## ✨ Funcionalidades
- Loop infinito de pomodoro
- Cores no terminal
- Flash na barra de tarefas (Windows) quando o ciclo está para acabar
- Configuração automática
- Zero complexidade: execute e trabalhe

---

## 📦 Compilação
```
cargo build --release
```
O binário final estará em:
```
target/release/pomoru
```

---

## 📜 Licença
MIT. Faça o que quiser.

---

Feito com 💙

