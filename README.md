# pomoru

Um pomodoro **extremamente simples** para o terminal. Sem distrações, sem interfaces gigantescas, sem dependências externas além do próprio binário. Só executar e usar.

## O que é o pomoru?

O pomoru é um timer pomodoro minimalista. Quando você executa o binário:

- Ele inicia imediatamente uma sessão de **trabalho**.
    
- Quando acaba, alterna automaticamente para **descanso**.
    
- Depois volta para **trabalho**.
    
- E segue alternando **infinitamente**, até você encerrar com `CTRL + C`.
    

Só isso.

---

## 🚀 Como usar

### Iniciar o pomodoro

```
pomoru
```

Isso começa imediatamente o ciclo **trabalho → descanso → trabalho → descanso...**
```powershell
.\pomoru.exe
CTRL + C to stop the pomodoro session.
[WORK] -> 00:57
```
### Acessar ajuda

```
pomoru h
```

Saída:
```
pomoru - CLI Pomodoro session
Usage: pomoru [OPTIONS] or just execute the binary to start a pomodoro session.
Options:
 h -> Show this message
 w -> Set work duration in minutes (default: 25)
 r -> Set rest duration in minutes (default: 5)
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
