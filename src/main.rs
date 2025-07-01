mod count;
mod format_output;

use clap::{Arg, ArgAction, Command};
use std::fs;
use std::io::{self, Read};

fn main() {
    let app = Command::new("wcrs")
        .version("0.1")
        .about("Contador como o wc do Linux")
        .arg(Arg::new("linhas").short('l').action(ArgAction::SetTrue))
        .arg(Arg::new("palavras").short('w').action(ArgAction::SetTrue))
        .arg(Arg::new("bytes").short('b').action(ArgAction::SetTrue))
        .arg(Arg::new("caracteres").short('c').action(ArgAction::SetTrue))
        .arg(Arg::new("arquivo").help("Arquivo para contar"))
        .get_matches();

    let conteudo = if let Some(arquivo) = app.get_one::<String>("arquivo") {
        fs::read_to_string(arquivo).expect("Erro ao ler arquivo")
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).expect("Erro ao ler entrada");
        buffer
    };

     let (total_linhas, total_palavras, total_bytes, total_caracteres) = (
        count::linhas(&conteudo),
        count::palavras(&conteudo),
        count::bytes(&conteudo),
        count::caracteres(&conteudo),
    );

let nenhuma_flag = 
    !app.get_flag("linhas") &&
    !app.get_flag("palavras") &&
    !app.get_flag("bytes") &&
    !app.get_flag("caracteres");

let mostrar = (
    if app.get_flag("linhas") || nenhuma_flag { Some(total_linhas) } else { None },
    if app.get_flag("palavras") || nenhuma_flag { Some(total_palavras) } else { None },
    if app.get_flag("bytes") || nenhuma_flag { Some(total_bytes) } else { None },
    if app.get_flag("caracteres") { Some(total_caracteres) } else { None },
);
    let nome_arquivo = app.get_one::<String>("arquivo").map(|s| s.as_str()).unwrap_or("");
    
    println!("{}", format_output::formatar_saida(
        mostrar.0,
        mostrar.1,
        mostrar.2,
        mostrar.3,
        nome_arquivo
    ));

}