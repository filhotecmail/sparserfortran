use argh::FromArgs;
use std::fs::{self,OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::env;
use std::thread;
use std::time::Duration;
use regex::Regex;
use chrono::Local;

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
            program_name, version, build_type, env::var("PROFILE").unwrap_or_else(|_| "Desconhecido".to_string())
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
    #[allow(dead_code)]
    ex: String,

    #[argh(option, description = "nome da função ou sub-rotina a ser extraída")]
    f: String,

    #[argh(positional, description = "caminho para o arquivo Fortran")]
    file: String,
}

struct FacadeApp {
    args: Args,
    program_name: String,
    version: String,
}

impl FacadeApp {
    fn new() -> Self {
        let args: Args = argh::from_env();
        let program_name = "SparserFortran".to_string(); 
        let version = "1.0.0".to_string();  

        let app = FacadeApp {
            args,
            program_name,
            version,
        };

        app.show_ascii_art(); 
        app.print_file_info();
        app
    }

    fn extract_module(file_path: &str, function_name: &str) -> io::Result<()> {
        let content = fs::read_to_string(file_path)?;
        let current_time = Local::now().format("%Y-%m-%d-%H%M%S").to_string();
        let backup_dir = "bkp";
        fs::create_dir_all(backup_dir)?;
        let backup_file_name = format!("{}/{}-{}.f90", backup_dir, current_time, function_name);
        let backup_file_path = Path::new(&backup_file_name);
    
        let re = Regex::new(&format!(r"\b{}\b", function_name))
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
    
        let mut inside_module = false;
        let mut module_content = String::new();
        let mut module_start_line = 0;
    
        // Armazenando o conteúdo do módulo
        let mut module_code = String::new();
    
        for (line_number, line) in content.lines().enumerate() {
            if line.contains("module ") && !inside_module {
                inside_module = true;
                module_start_line = line_number + 1;
            }
    
            if inside_module {
                module_content.push_str(line);
                module_content.push('\n'); // Mantém a quebra de linha
                module_code.push_str(line); // Mantém a quebra de linha ao escrever no backup
                module_code.push('\n'); // Garante que a quebra de linha seja adicionada ao backup
    
                if line.contains("end module") {
                    inside_module = false;
                }
            }
        }
    
        if module_content.contains(&format!("module {}", function_name)) {
            // Abrir arquivo de backup para escrita
            let mut backup_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true) 
                .open(backup_file_path)?;    
      
            backup_file.write_all(module_code.as_bytes())?;
    
            let updated_content = re.replace_all(&content, &format!(
                " ! Code removed by sparseFortran application on date {}",
                current_time
            ));
    
            Self::confirm_removal(file_path, &content, &module_content)
        } else {
            println!("Módulo '{}' não encontrado!", function_name);
            Ok(())
        }
    }

    fn extract_function(file_path: &str, function_name: &str) -> io::Result<()> {
        let content = fs::read_to_string(file_path)?;
        let function_pattern = format!(r"function {}\([^\)]*\)[\s\S]*?end function {}", function_name, function_name);
        let re = Regex::new(&function_pattern).unwrap();

        if let Some(function_match) = re.find(&content) {
            println!("\nFunção '{}' encontrada!", function_name);
            println!("Linha: {}", content[..function_match.start()].lines().count() + 1);
            println!("Conteúdo completo da função:\n{}", &content[function_match.start()..function_match.end()]);

            Self::confirm_removal(file_path, &content, &content[function_match.start()..function_match.end()])
        } else {
            println!("Função '{}' não encontrada!", function_name);
            Ok(())
        }
    }

    fn extract_subroutine(file_path: &str, func_name: &str) -> io::Result<()> {
        let content = fs::read_to_string(file_path)?;
        let pattern = format!(r"subroutine {}\([^\)]*\)[\s\S]*?end subroutine {}", func_name, func_name);
        let re = Regex::new(&pattern).unwrap();
    
        if let Some(sub_match) = re.find(&content) {
            println!("\nSub-rotina '{}' encontrada!", func_name);
            println!("Linha: {}", content[..sub_match.start()].lines().count() + 1);
            println!("Conteúdo completo da sub-rotina:\n{}", &content[sub_match.start()..sub_match.end()]);
    
            println!("\nDeseja remover a sub-rotina?");
            println!("  [Y] para remover, [ALL] para remover e referências, [n] para cancelar");
    
            let mut input = String::new();
            io::stdin().read_line(&mut input)?; 
            let choice = input.trim().to_lowercase();
    
            match choice.as_str() {
                "y" => {
                     
                    let content = fs::read_to_string(file_path)?;    
                    let current_time = Local::now().format("%Y-%m-%d as %H:%M:%S").to_string();

                    let backup_dir = "bkp";
                    fs::create_dir_all(backup_dir)?;
                    let backup_file_name = format!("{}/{}-{}.f90", backup_dir, current_time, func_name);
                    let backup_file_path = Path::new(&backup_file_name);

                    for sub_match in re.find_iter(&content) {
                        let matched_content = &content[sub_match.start()..sub_match.end()]; 
                        fs::write(backup_file_path, matched_content.as_bytes())?;                    
                    }

                    let updated_content = re.replace_all(&content, &format!(
                        " ! Code removed by sparseFortran application on date {}",
                        current_time
                    ));                 
                    fs::write(file_path, updated_content.as_bytes())?;
                    println!("\x1b[32mSub-rotina '{}' removida com sucesso!\x1b[0m", func_name);
                },
                "all" => {

                    let content = fs::read_to_string(file_path)?;
                    let current_time = Local::now().format("%Y-%m-%d as %H:%M:%S").to_string();
                    
                    let backup_dir = "bkp";
                    fs::create_dir_all(backup_dir)?;
                    let backup_file_name = format!("{}/{}-{}.f90", backup_dir, current_time, func_name);
                    let backup_file_path = Path::new(&backup_file_name);

                    for sub_match in re.find_iter(&content) {
                        let matched_content = &content[sub_match.start()..sub_match.end()]; 
                        fs::write(backup_file_path, matched_content.as_bytes())?;                    
                    }
                    
                    let updated_content = re.replace_all(&content, &format!(
                        " ! Code removed by sparseFortran application on date {}",
                        current_time
                    )); 

                    let call_pat = format!(r"\bcall\s+{}\b", func_name);
                    let re_call = Regex::new(&call_pat).unwrap();
                    let final_content = re_call.replace_all(&updated_content,&format!(
                        " ! Code removed by sparseFortran application on date {}\n",
                        current_time
                    )); 
                    fs::write(file_path, final_content.as_bytes())?;
                    println!("\x1b[32mSub-rotina '{}' e chamadas removidas com sucesso!\x1b[0m", func_name);

                },
                _ => {
                    println!("Operação cancelada.");
                }
            }
        } else {
            println!("\x1b[31m[ error ] Sub-rotina '{}' não encontrada!\x1b[0m", func_name);

        }
    
        Ok(())
    }   

    fn confirm_removal(file_path: &str, content: &str, target_content: &str) -> io::Result<()> {
        println!("\nVocê deseja remover o conteúdo? [Y] para remover, [n] para cancelar");
    
        let mut input = String::new();
        io::stdin().read_line(&mut input)?; 
        let input = input.trim().to_lowercase(); 
    
        match input.as_str() {
            "y" => {
                let updated_content = content.replace(target_content, ""); 
                fs::write(file_path, updated_content.as_bytes())?;
                println!("\x1b[32mConteúdo removido com sucesso!\x1b[0m");

            },
            _ => {
                println!("Operação cancelada.");
            }
        }
    
        Ok(()) 
    }

    fn initialize(self) -> io::Result<Self> {
        let content = std::fs::read_to_string(&self.args.file)?;
    
        let pattern = format!(r"\b(subroutine|function|module)\s+{}\b", &self.args.f);
        let re = Regex::new(&pattern).unwrap();
    
        match re.captures(&content) {
            Some(captures) => {
                let keyword = captures.get(1).unwrap().as_str();
                match keyword {
                    "subroutine" => Self::extract_subroutine(&self.args.file, &self.args.f)?,
                    "module" => Self::extract_module(&self.args.file, &self.args.f)?,
                    "function" => Self::extract_function(&self.args.file, &self.args.f)?,
                    _ => unreachable!(),
                }
            }
            None => {
                eprintln!("\x1b[31m[error] Método, função ou subrotina '{}' não encontrado na unidade '{}'\x1b[0m", self.args.f, self.args.file);
                return Err(io::Error::new(io::ErrorKind::NotFound, "Elemento não encontrado"));
            }
        }
    
        Ok(self)  
    }

    fn show_ascii_art(&self) {
        let build_type = env::var("PROFILE").unwrap_or_else(|_| "Release".to_string());
        let art = AsciiArt::new(&self.program_name, &build_type, &self.version);

        let mut color = StandardStream::stdout(ColorChoice::Always);
        color.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();  
        println!("{}", art.render());
        color.reset().unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    fn print_file_info(&self) {
        let metadata = match fs::metadata(&self.args.file) {
            Ok(meta) => meta,
            Err(_) => {
                eprintln!("\x1b[31m[ error ] Erro ao acessar o arquivo '{}'\x1b[0m", self.args.file);
                return;
            }
        };
    
        let file_name = self.args.file.split('/').last().unwrap_or(&self.args.file);
        let extension = file_name.split('.').last().unwrap_or("Desconhecido");
        let file_size = metadata.len();
        
        let lines = match fs::read_to_string(&self.args.file) {
            Ok(content) => content.lines().count(),
            Err(_) => {
                eprintln!("\x1b[31m[ error ]] Erro ao ler o conteúdo do arquivo '{}'\x1b[0m", self.args.file);
                return;
            }
        };
    
        println!("\nNome do arquivo: {}", file_name);
        println!("Caminho do Arquivo: {}", self.args.file);
        println!("Extensão do arquivo: {}", extension);
        println!("Tamanho do arquivo: {} bytes", file_size);
        println!("Número de linhas: {}\n", lines);

    }
    
}

fn main() {    
    FacadeApp::new().initialize().unwrap(); 
}
