# zxc-sudoku

Sudoku no terminal feito com Rust e ratatui.

## Funcionalidades

- Geração automática de puzzles com backtracking
- 3 níveis de dificuldade (Fácil, Médio, Difícil)
- Sistema de dicas
- Detecção em tempo real de duplicatas
- Highlighting de linha/coluna/bloco
- Interface colorida no terminal

## Pré-requisitos

- Rust 2024 ou superior

## Instalação e execução

```bash
# Clonar o repositório
git clone <url-do-repositorio>
cd zxc-sudoku

# Compilar e executar (debug)
cargo run

# Compilar e executar (release, otimizado)
cargo run --release
```

## Controles

| Tecla | Ação |
|---|---|
| Setas | Mover cursor |
| `1`-`9` | Inserir número |
| `Backspace`/`Delete` | Apagar célula |
| `H` | Dica (revela resposta) |
| `R` | Novo jogo |
| `D` | Alterar dificuldade |
| `Q` / `Esc` | Sair |

## Estrutura do projeto

```
src/
├── main.rs    # Ponto de entrada e loop de eventos
├── app.rs     # Estado da aplicação
├── game.rs    # Lógica Sudoku
└── ui.rs      # Renderização no terminal
```

## Licença

MIT
