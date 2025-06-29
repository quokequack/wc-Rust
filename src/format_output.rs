pub fn formatar_saida(
    linhas: Option<usize>,
    palavras: Option<usize>,
    bytes: Option<usize>,
    caracteres: Option<usize>,
    arquivo: &str,
) -> String {
    let mut partes = Vec::new();

    if let Some(l) = linhas {
        partes.push(format!("Linhas: {}", l));
    }
    if let Some(p) = palavras {
        partes.push(format!("Palavras: {}", p));
    }
    if let Some(b) = bytes {
        partes.push(format!("Bytes: {}", b));
    }
    if let Some(c) = caracteres {
        partes.push(format!("Caracteres: {}", c));
    }

    let mut resultado = partes.join(" | ");
    
    if !arquivo.is_empty() {
        resultado.push_str(" | Arquivo: ");
        resultado.push_str(arquivo);
    }

    resultado
}