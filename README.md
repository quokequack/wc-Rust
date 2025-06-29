# Documentação do Projeto wcrs (Clone do wc em Rust)

## Visão Geral
O wcrs é um clone simplificado do comando `wc` do Linux, desenvolvido em Rust. Ele conta linhas, palavras, bytes e caracteres de arquivos ou entrada padrão, com saída formatada.

## Arquivo `count.rs`

### Funções de Contagem

#### `linhas(texto: &str) -> usize`
```rust
pub fn linhas(texto: &str) -> usize {
    texto.lines().count()
}
```
- **Função**: Conta o número de linhas em um texto
- **Método usado**: `lines()` divide o texto em linhas e `count()` conta os elementos
- **Comportamento**: Cada quebra de linha (`\n`) incrementa o contador

#### `palavras(texto: &str) -> usize`
```rust
pub fn palavras(texto: &str) -> usize {
    texto.split_whitespace().count()
}
```
- **Função**: Conta palavras em um texto
- **Método usado**: `split_whitespace()` divide por espaços em branco (inclui múltiplos espaços/tabs)
- **Comportamento**: Sequências de whitespace são tratadas como um único separador

#### `bytes(texto: &str) -> usize`
```rust
pub fn bytes(texto: &str) -> usize {
    texto.as_bytes().len()
}
```
- **Função**: Conta o número de bytes
- **Método usado**: `as_bytes()` converte para bytes UTF-8 e `len()` conta
- **Importante**: Diferente de caracteres (ex: 'á' = 2 bytes)

#### `caracteres(texto: &str) -> usize`
```rust
#[allow(dead_code)]
pub fn caracteres(texto: &str) -> usize {
    texto.chars().count()
}
```
- **Função**: Conta caracteres Unicode
- **Método usado**: `chars()` itera sobre chars Unicode
- **Nota**: Marcada como `#[allow(dead_code)]` pois pode não ser usada em todas as execuções

### Testes Unitários
```rust
#[cfg(test)]
mod testes {
    use super::*;
    
    #[test]
    fn teste_linhas() { /* ... */ }
    
    #[test]
    fn teste_palavras() { /* ... */ }
    
    #[test]
    fn teste_bytes() { /* ... */ }
    
    #[test]
    fn teste_caracteres() { /* ... */ }
}
```
- Verificam o correto funcionamento de cada função de contagem
- Casos testados incluem Unicode e múltiplas linhas

## Arquivo `format_output.rs`

### Função `formatar_saida`
```rust
pub fn formatar_saida(
    linhas: Option<usize>,
    palavras: Option<usize>,
    bytes: Option<usize>,
    caracteres: Option<usize>,
    arquivo: &str,
) -> String
```
- **Objetivo**: Formatar os resultados para exibição
- **Parâmetros**: 
  - Valores opcionais (Option) para cada métrica
  - Nome do arquivo como string slice

#### Lógica Interna:
1. **Coleta de Partes**:
   ```rust
   let mut partes = Vec::new();
   
   if let Some(l) = linhas {
       partes.push(format!("Linhas: {}", l));
   }
   // Padrão repetido para outras métricas
   ```
   - Cria um vetor dinâmico para armazenar partes do resultado
   - `if let` verifica se o valor existe (Some) antes de formatar

2. **Junção das Partes**:
   ```rust
   let mut resultado = partes.join(" | ");
   ```
   - Une as partes com separador " | "

3. **Adição do Nome do Arquivo**:
   ```rust
   if !arquivo.is_empty() {
       resultado.push_str(" | Arquivo: ");
       resultado.push_str(arquivo);
   }
   ```
   - Adiciona o arquivo apenas se não for string vazia

## Arquivo `main.rs`

### Configuração CLI com Clap
```rust
let app = Command::new("wcrs")
    .version("0.1")
    .about("Contador como o wc do Linux")
    .arg(Arg::new("linhas").short('l').action(ArgAction::SetTrue))
    // ... outras flags ...
    .get_matches();
```
- **Flags**:
  - `-l`: Contar linhas
  - `-w`: Contar palavras
  - `-b`: Contar bytes
  - `-c`: Contar caracteres
- `ArgAction::SetTrue`: Flags booleanas que ativam funcionalidades

### Leitura de Entrada
```rust
let conteudo = if let Some(arquivo) = app.get_one::<String>("arquivo") {
    fs::read_to_string(arquivo).expect("Erro ao ler arquivo")
} else {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Erro ao ler entrada");
    buffer
};
```
- Lê de arquivo (se especificado) ou da entrada padrão (stdin)
- Tratamento de erro básico com `expect`

### Lógica Principal
1. **Cálculo das Métricas**:
   ```rust
   let (total_linhas, total_palavras, total_bytes, total_caracteres) = (
       count::linhas(&conteudo),
       count::palavras(&conteudo),
       count::bytes(&conteudo),
       count::caracteres(&conteudo),
   );
   ```
   - Calcula todas as métricas uma vez (otimização)

2. **Seleção do que Mostrar**:
   ```rust
   let mostrar = (
       if app.get_flag("linhas") || !app.args_present() { Some(total_linhas) } else { None },
       // ... padrão repetido ...
   );
   ```
   - Se nenhuma flag: mostra linhas, palavras e bytes (padrão wc)
   - Flags específicas ativam métricas correspondentes

3. **Formatação Final**:
   ```rust
   println!("{}", format_output::formatar_saida(...));
   ```
   - Chama a função de formatação com os valores selecionados

## Fluxo do Programa
1. Parseia argumentos CLI
2. Decide fonte de entrada (arquivo ou stdin)
3. Calcula todas as métricas
4. Filtra o que mostrar baseado nas flags
5. Formata e exibe o resultado

## Como Executar o Projeto wcrs

### Pré-requisitos
1. **Rust instalado**: Você precisa do Rust e do Cargo (gerenciador de pacotes do Rust)
   ```bash
   # Verifique a instalação
   rustc --version
   cargo --version
   ```

2. **Clone o repositório** (se aplicável):
   ```bash
   git clone [URL_DO_REPOSITÓRIO]
   cd wcrs
   ```

### Compilação
```bash
# Compilar em modo debug (para desenvolvimento)
cargo build

# Compilar em modo release (para uso final)
cargo build --release
```

### Execução Básica
O executável será gerado em:
- Modo debug: `target/debug/wcrs`
- Modo release: `target/release/wcrs`

#### Opção 1: Executar diretamente com Cargo
```bash
# Formato geral:
cargo run -- [OPÇÕES] [ARQUIVO]

# Exemplos:
cargo run -- exemplo.txt               # Comportamento padrão (linhas, palavras, bytes)
cargo run -- -l -w exemplo.txt        # Apenas linhas e palavras
cargo run -- -c                       # Apenas caracteres (lê do stdin)
```

#### Opção 2: Executar o binário compilado
```bash
# Primeiro compile
cargo build --release

# Depois execute
./target/release/wcrs [OPÇÕES] [ARQUIVO]
```

### Modos de Uso

#### 1. Ler de um arquivo
```bash
wcrs [OPÇÕES] arquivo.txt
```
Exemplo:
```bash
wcrs -l -c documento.md  # Mostra apenas contagem de linhas e caracteres
```

#### 2. Ler da entrada padrão (stdin)
```bash
comando | wcrs [OPÇÕES]
```
Exemplos:
```bash
echo "Olá mundo" | wcrs -w       # Conta palavras da string
cat arquivo.txt | wcrs -l -b     # Conta linhas e bytes
```

#### 3. Comportamento padrão (sem flags)
```bash
wcrs arquivo.txt
```
Mostra (nesta ordem):
1. Número de linhas
2. Número de palavras
3. Número de bytes
4. Nome do arquivo (se aplicável)

### Flags Disponíveis
| Flag | Descrição                     | Exemplo de Uso       |
|------|-------------------------------|----------------------|
| `-l` | Conta linhas                  | `wcrs -l arquivo`    |
| `-w` | Conta palavras                | `wcrs -w arquivo`    |
| `-b` | Conta bytes                   | `wcrs -b arquivo`    |
| `-c` | Conta caracteres (Unicode)    | `wcrs -c arquivo`    |

### Exemplos Completos

1. **Contagem completa (padrão wc)**:
   ```bash
   cargo run -- exemplo.txt
   # Saída: Linhas: 42 | Palavras: 230 | Bytes: 1450 | Arquivo: exemplo.txt
   ```

2. **Apenas linhas e palavras**:
   ```bash
   cargo run -- -l -w exemplo.txt
   # Saída: Linhas: 42 | Palavras: 230 | Arquivo: exemplo.txt
   ```

3. **Ler da entrada padrão**:
   ```bash
   echo -e "Olá mundo\nEsta é uma frase" | cargo run -- -l -w
   # Saída: Linhas: 2 | Palavras: 5
   ```

4. **Mostrar apenas caracteres Unicode**:
   ```bash
   cargo run -- -c exemplo_unicode.txt
   # Saída: Caracteres: 1420 | Arquivo: exemplo_unicode.txt
   ```

### Dicas Rápidas
1. Para ajuda:
   ```bash
   cargo run -- --help
   ```

2. Para testar com dados imediatos:
   ```bash
   echo "Teste rápido" | cargo run -- -w
   ```

3. Para compilar e instalar globalmente:
   ```bash
   cargo install --path .
   # Depois pode usar diretamente:
   wcrs [OPÇÕES] [ARQUIVO]
   ```

## Dependências
- `clap`: Para parsing de argumentos CLI (versão 4.x)