use argh::FromArgs;
use std::fs;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use strsim::levenshtein;
use regex::Regex;
use std::io::{self}; 
use std::thread;
use std::time::Duration;
use std::env;

struct AsciiArt {
    art: String,
}

impl AsciiArt {
    fn new(program_name: &str, build_type: &str, version: &str) -> Self {
        let art = format!(
            "
+---------------------------------------------------------------------------------------------+
|  Bem-vindo ao {} - Versão: {}                                              |
|  Compilação: {}                                                                        |
|  Ambiente de execução: {}                                                         |
|  Descrição: Extrai métodos de um arquivo Fortran e permite sua remoção ou modificação.      |
|  Este programa facilita o processamento de código Fortran, extraindo sub-rotinas e módulos, |
|  focando em performance e clareza na modificação do código.                                 |
|                                                                                             |
|  Programador: Carlos A. Dias da Silva F.                                                    |
|  Contato: filhotecmail@gmail.com                                                            |
|                                                                                             |
|  Licença: GNU General Public License (GPL)                                                  |
|  Este programa é software livre; você pode redistribuí-lo e/ou modificá-lo sob os termos da |
|  Licença Pública Geral GNU, conforme publicada pela Free Software Foundation, na versão 3,  |
|  ou (a sua escolha) qualquer versão posterior.                                              |
|  Este programa é distribuído na esperança de que seja útil, mas SEM NENHUMA GARANTIA; sem   |
|  mesmo a garantia implícita de COMERCIABILIDADE ou ADEQUAÇÃO A UM DETERMINADO FIM. Veja a   |
|  Licença Pública Geral GNU para mais detalhes.                                              |
+---------------------------------------------------------------------------------------------+
",
            program_name, get_version(), build_type, env::var("PROFILE").unwrap_or_else(|_| "Desconhecido".to_string())
        );

        AsciiArt { art }
    }

    fn render(&self) -> &str {
        &self.art
    }
}

#[derive(FromArgs)]
/// Estrutura para lidar com os argumentos da linha de comando.
struct Args {
    #[argh(option, description = "método a ser usado")]
    ex: String,

    #[argh(option, description = "nome da função ou sub-rotina a ser extraída")]
    f: String,

    #[argh(positional, description = "caminho para o arquivo Fortran")]
    file: String,
}

fn extract_module(file_path: &str, function_name: &str) -> io::Result<()> {
    let content = fs::read_to_string(file_path)?;
    let mut inside_module = false;
    let mut module_content = String::new();
    let mut module_line = 0;

    for (index, line) in content.lines().enumerate() {
        if line.contains("module ") && !inside_module {
            inside_module = true;
            module_line = index + 1;
        }

        if inside_module {
            module_content.push_str(line);
            module_content.push('\n');
            if line.contains("end module") {
                inside_module = false;
            }
        }
    }

    if module_content.contains(&format!("module {}", function_name)) {
        println!("\nMódulo '{}' encontrado!", function_name);
        println!("Linha: {}", module_line);
        println!("Conteúdo completo do módulo:\n{}", module_content);

        println!("\nVocê deseja remover o módulo inteiro?");
        println!("  [Y] para remover o módulo, [n] para cancelar");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "y" => {
                let updated_content: String = content.lines()
                    .enumerate()
                    .filter(|(_, line)| {
                        let line_str = line.trim();
                        !(line_str.contains("module") || line_str.contains("end module"))
                    })
                    .map(|(_, line)| line)
                    .collect::<Vec<&str>>()
                    .join("\n");

                fs::write(file_path, updated_content.as_bytes())?;
                println!("Módulo '{}' removido com sucesso!", function_name);
            },
            _ => {
                println!("Operação cancelada.");
            }
        }
    } else {
        println!("Módulo '{}' não encontrado!", function_name);
    }

    Ok(())
}

fn extract_available_functions(content: &str) -> Vec<String> {
    let mut functions = Vec::new();
    let re_module = Regex::new(r"\bmodule\s+(\w+)").unwrap();
    let re_function = Regex::new(r"\bfunction\s+(\w+)").unwrap();
    let re_subroutine = Regex::new(r"\bsubroutine\s+(\w+)").unwrap();
 
    for cap in re_module.captures_iter(content) {
        functions.push(cap[1].to_string());
    }
    for cap in re_function.captures_iter(content) {
        functions.push(cap[1].to_string());
    }
    for cap in re_subroutine.captures_iter(content) {
        functions.push(cap[1].to_string());
    }

    functions
}

fn find_closest_match(function_name: &str, available_functions: &[String]) -> Option<String> {
    let mut closest_match: Option<String> = None;
    let mut min_distance = usize::MAX;

    for function in available_functions {
        let distance = levenshtein(function_name, function);
        if distance < min_distance {
            min_distance = distance;
            closest_match = Some(function.clone());
        }
    }

    closest_match
}

fn print_error_with_suggestion(function_name: &str, suggestions: Vec<String>) {
    let mut color = StandardStream::stdout(ColorChoice::Always);

    color.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
    println!("[ erro ] -> Função '{}' não encontrada!", function_name);
    for suggestion in suggestions {
        println!("[ sugestão ] -> Você quis dizer '{}'? ", suggestion);
    }
    color.reset().unwrap();
}


fn extract_subroutine(file_path: &str, function_name: &str) -> io::Result<()> {
    let content = fs::read_to_string(file_path)?;
    let subroutine_pattern = format!(r"subroutine {}\([^\)]*\)[\s\S]*?end subroutine {}", function_name, function_name);
    let re = Regex::new(&subroutine_pattern).unwrap();    
    
    if let Some(subroutine_match) = re.find(&content) {
        println!("\nSub-rotina '{}' encontrada!", function_name);
        println!("Linha: {}", content[..subroutine_match.start()].lines().count() + 1);
        println!("Conteúdo completo da sub-rotina:\n{}", &content[subroutine_match.start()..subroutine_match.end()]);

        println!("\nVocê deseja remover a sub-rotina inteira?");
        println!("  [Y] para remover a sub-rotina, [ALL] para remover a sub-rotina e todas as referências, [n] para cancelar");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "y" => {
                let updated_content = re.replace_all(&content, "");
                fs::write(file_path, updated_content.as_bytes())?;
                println!("Sub-rotina '{}' removida com sucesso!", function_name);
            },
            "all" => {
                let updated_content = re.replace_all(&content, "");            
                let call_pattern = format!(r"\bcall\s+{}\b", function_name); 
                let re_call = Regex::new(&call_pattern).unwrap();
                let final_content = re_call.replace_all(&updated_content, "");
            
                fs::write(file_path, final_content.as_bytes())?;
                println!("Sub-rotina '{}' e todas as referências (incluindo chamadas) removidas com sucesso!", function_name);
            },
            _ => {
                println!("Operação cancelada.");
            }
        }
    } else {
       
    }

    Ok(())
}

fn extract_function(file_path: &str, function_name: &str) -> io::Result<()> {
    if file_path.contains(function_name) {
        extract_module(file_path, function_name)?
    } else {
        extract_subroutine(file_path, function_name)?
    }

    Ok(())
}

fn extract_fn(file_path: &str, function_name: &str) -> io::Result<()> {
    let content = fs::read_to_string(file_path)?;
    let function_pattern = format!(r"function {}\([^\)]*\)[\s\S]*?end function {}", function_name, function_name);
    let re = Regex::new(&function_pattern).unwrap();

    if let Some(function_match) = re.find(&content) {
        println!("\nFunção '{}' encontrada!", function_name);
        println!("Linha: {}", content[..function_match.start()].lines().count() + 1);
        println!("Conteúdo completo da função:\n{}", &content[function_match.start()..function_match.end()]);

        println!("\nVocê deseja remover a função inteira?");
        println!("  [Y] para remover a função, [ALL] para remover a função e todas as referências, [n] para cancelar");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "y" => {
                let updated_content = re.replace_all(&content, "");
                fs::write(file_path, updated_content.as_bytes())?;
                println!("Função '{}' removida com sucesso!", function_name);
            },
            "all" => {
                let updated_content = re.replace_all(&content, "");
                let call_pattern = format!(r"\bcall\s+{}\b", function_name);
                let re_call = Regex::new(&call_pattern).unwrap();
                let final_content = re_call.replace_all(&updated_content, "");

                fs::write(file_path, final_content.as_bytes())?;
                println!("Função '{}' e todas as referências (incluindo chamadas) removidas com sucesso!", function_name);
            },
            _ => {
                println!("Operação cancelada.");
            }
        }
    } else {
        let available_functions = vec!["function1".to_string(), "function2".to_string()]; // Exemplo, deve ser substituído pelas funções reais
        print_error_with_suggestion(function_name, available_functions);
    }

    Ok(())
}

fn show_ascii_art(program_name: &str, version: &str) {
    let build_type = env::var("PROFILE").unwrap_or_else(|_| "Release".to_string());
    let art = AsciiArt::new(program_name, &build_type, version);

    let mut color = StandardStream::stdout(ColorChoice::Always);
    color.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();  // Definindo a cor verde
    println!("{}", art.render());
    color.reset().unwrap();
}

fn print_file_info(file_path: &str) -> io::Result<()> {
    let metadata = fs::metadata(file_path)?;
    let file_name = file_path.split('/').last().unwrap_or(file_path);
    let extension = file_name.split('.').last().unwrap_or("Desconhecido");
    let file_size = metadata.len();
    let lines = fs::read_to_string(file_path)?.lines().count();

    println!("\nNome do arquivo: {}", file_name);
    println!("Caminho do Arquivo: {}", file_path);
    println!("Extensão: {}", extension);
    println!("Tamanho do arquivo em bytes: {}", file_size);
    println!("Número de linhas do arquivo: {}", lines);

    Ok(())
}

fn get_version() -> String {
    let version = env!("CARGO_PKG_VERSION"); 
    version.to_string()
}

fn main() -> io::Result<()> {
    let args: Args = argh::from_env();
    let version = "1.0.0"; 

    show_ascii_art("FortranExtractor", version);
    thread::sleep(Duration::from_secs(3));
    print_file_info(&args.file)?;
    extract_function(&args.file, &args.f)
}
