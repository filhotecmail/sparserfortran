# SparserFortran

**SparserFortran** é um programa desenvolvido para extrair métodos de arquivos Fortran e permitir sua remoção ou modificação. Ele facilita o processamento de código Fortran, extraindo sub-rotinas e módulos, e oferece uma maneira eficiente de modificar o código, visando desempenho e clareza.

## Funcionalidades

- **Extração de Módulos:** Permite extrair módulos de arquivos Fortran.
- **Remoção de Funções e Sub-rotinas:** Possui a capacidade de remover funções e sub-rotinas de arquivos Fortran.
- **Backup e Registro de Modificações:** Realiza backups antes de remover qualquer código e adiciona um comentário no código removido com a data e hora.
- **Interface de Linha de Comando:** Controlado via linha de comando, permitindo automação e fácil integração.

## Tecnologias

- **Rust** - O programa foi desenvolvido em Rust, garantindo performance e segurança.
- **Regex** - Usado para buscar e manipular os conteúdos dos arquivos Fortran.
- **Chrono** - Para manipulação de datas e horários, usado na criação de backups e no registro das alterações.

## Instalação

### Dependências

1. **Rust:** Este projeto foi desenvolvido em Rust. Para instalar o Rust, siga as instruções em [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

### Compilação

Para compilar o projeto, use o cargo (gerenciador de pacotes do Rust) na raiz do projeto:

```bash
cargo build --release
