pub fn linhas(texto: &str) -> usize {
    texto.lines().count()
}

pub fn palavras(texto: &str) -> usize {
    texto.split_whitespace().count()
}

pub fn bytes(texto: &str) -> usize {
    texto.as_bytes().len()
}

#[allow(dead_code)]
pub fn caracteres(texto: &str) -> usize {
    texto.chars().count()
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn teste_linhas() {
        assert_eq!(linhas("linha 1\nlinha 2\n"), 2);
    }

    #[test]
    fn teste_palavras() {
        assert_eq!(palavras("uma duas três"), 3);
    }

    #[test]
    fn teste_bytes() {
        assert_eq!(bytes("abc"), 3);
    }

    #[test]
    fn teste_caracteres() {
        assert_eq!(caracteres("áé"), 2);
    }
}