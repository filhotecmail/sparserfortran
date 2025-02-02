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


/// Macro para formatar mensagens, substituindo os placeholders `{0}`, `{1}`, etc., pelos valores fornecidos.
///
/// # Exemplo:
/// ```rust
/// let formatted_message = format_message!(
///     "Hello, {0}! Welcome to {1}. You have {2} new messages.",
///     "Alice", 
///     "Rustland", 
///     5
/// );
/// assert_eq!(formatted_message, "Hello, Alice! Welcome to Rustland. You have 5 new messages.");
/// ```
///
/// # Parâmetros:
/// - `$message`: A mensagem que contém os placeholders `{0}`, `{1}`, etc., que serão substituídos.
/// - `$($val:expr),*`: Uma quantidade variável de valores que substituirão os placeholders.
///
/// # Retorna:
/// Retorna a string formatada com os valores substituídos.
macro_rules! format_message {
    ($message:expr, $($val:expr),*) => {{
        let mut message = $message.to_string();  
        let values = vec![$(String::from($val)),*];  
        for (i, value) in values.iter().enumerate() {
            message = message.replace(&format!("{{{}}}", i), &value); 
        }
        message  
    }};
}  

struct AsciiArt {
    art: String,
}

impl AsciiArt {
    fn new(program_name: &str, buildtype: &str, version: &str) -> Self {
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
            program_name, version, buildtype, env::var("PROFILE").unwrap_or_else(|_| "Release".to_string())
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
    #[argh(option, description = "method to be used")]
    #[allow(dead_code)]
    ex: String,
    
    #[argh(option, description = "name of the function or subroutine to be extracted")]
    f: String,
    
    #[argh(positional, description = "path to the Fortran file")]
    file: String,
    
}

struct Messages;

impl Messages {
    const M001_CONFIRM_REMOVE_MSG: &str                    = "\x1b[32m\nDo you want to remove the content? [Y] to remove, [n] to cancel\x1b[0m";
    const M002_SUCCESSFUL_REMOVAL_COLORED: &'static str    = "\x1b[32m[success] Content successfully removed!\x1b[0m";
    const M003_CODEREMOVED_MSG: &'static str               = " ! Code removed by sparseFortran application on date {0} ";
    const M004_FUNCTION_FOUND: &'static str                = "\x1b[32m[success] Function '{0}' found in the file '{1}'\x1b[0m";
    const M005_SUBROUTINE_REMOVED_SUCESS : &'static str    = "\x1b[32m[success] Subroutine {0} successfully removed!\x1b[0m";
    
    const E001_METHODFUNCIONORSUBR_NOTFOUND : &'static str = "\x1b[31m[error] Method, function, or subroutine '{0}' not found in the file '{1}'\x1b[0m";
    const E002_MODULE_NOTFOUND : &'static str              = "\x1b[31m[error] Module '{0}' not found in the file '{1}'\x1b[0m";
    const E003_FUNCTION_NOTFOUND: &'static str             = "\x1b[31m[error] Function '{0}' not found in the file '{1}'\x1b[0m";
    pub const E004_SUBROUTINE_NOTFOUND: &str               = "\x1b[31m[error] Subroutine '{0}' not found in the file '{1}'\x1b[0m"; 

    /// Formata uma mensagem substituindo os placeholders `{0}`, `{1}`, etc., pelos valores fornecidos.
    /// Criei isso para me facilitar a usar como chamada estatica que vai usar a macro 
    /// 
    /// # Exemplo:
    /// ```rust
    /// let formatted_message = format_message!(
    ///     Messages::M004_FUNCTION_FOUND,
    ///     "my_function",
    ///     "my_file.f90"
    /// );
    /// assert_eq!(formatted_message, "\x1b[32m[success] Function 'my_function' found in the file 'my_file.f90'\x1b[0m");
    /// ```
    pub fn format_message(message: &str, args: &[&str]) -> String {
        let mut formatted_message = message.to_string();
        for (i, arg) in args.iter().enumerate() {
            formatted_message = formatted_message.replace(&format!("{{{}}}", i), arg);
        }
        formatted_message
    }
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
        let c_time = Local::now().format("%Y-%m-%d-%H%M%S").to_string();
        let backup_dir = "bkp";
        
        fs::create_dir_all(backup_dir)?;
        let bkpfl_name = format!("{}/{}-{}.f90", backup_dir, c_time, function_name);
        let bkpfl_path = Path::new(&bkpfl_name);
    
        let re = Regex::new(&format!(r"\b{}\b", function_name))
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
    
        let mut inside_module = false;
        let mut module_content = String::new();
        let mut module_code = String::new();
    
        for (_, line) in content.lines().enumerate() {
            if line.contains("module ") && !inside_module {
                inside_module = true;
            }
    
            if inside_module {
                module_content.push_str(line);
                module_content.push('\n');
                module_code.push_str(line);
                module_code.push('\n'); 
    
                if line.contains("end module") {
                    inside_module = false;
                }
            }
        }
    
        if module_content.contains(&format!("module {}", function_name)) {          
            let mut backup_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true) 
                .open(bkpfl_path)?;
      
            backup_file.write_all(module_code.as_bytes())?;
    
            re.replace_all(&content, &format_message!(
                &Messages::M003_CODEREMOVED_MSG,
                c_time
            ));
    
            Self::confirm_removal(file_path, &content, &module_content)
        } else {
            eprintln!("{}", format_message!( Messages::E002_MODULE_NOTFOUND, function_name,file_path));

            Ok(())
        }
    }
    
    fn extract_function(file_path: &str, function_name: &str) -> io::Result<()> {
        let content = fs::read_to_string(file_path)?;
        let function_pattern = format!(r"function {}\([^\)]*\)[\s\S]*?end function {}", function_name, function_name);
        let re = Regex::new(&function_pattern).unwrap();

        if let Some(function_match) = re.find(&content) {

            eprintln!("{}", format_message!( Messages::M004_FUNCTION_FOUND, function_name,file_path));

            println!("Line: {}", content[..function_match.start()].lines().count() + 1);
            println!("Full content of the function:\n{}", &content[function_match.start()..function_match.end()]);

            Self::confirm_removal(file_path, &content, &content[function_match.start()..function_match.end()])
        } else {
            eprintln!("{}", format_message!( Messages::E003_FUNCTION_NOTFOUND, function_name,file_path));
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
                    let c_time = Local::now().format("%Y-%m-%d as %H:%M:%S").to_string();

                    let backup_dir = "bkp";
                    fs::create_dir_all(backup_dir)?;
                    let bkpfl_name = format!("{}/{}-{}.f90", backup_dir, c_time, func_name);
                    let bkpfl_path = Path::new(&bkpfl_name);

                    for sub_match in re.find_iter(&content) {
                        let matched_content = &content[sub_match.start()..sub_match.end()]; 
                        fs::write(bkpfl_path, matched_content.as_bytes())?;                    
                    }

                    let updated_content = re.replace_all(&content, Messages::format_message(  &Messages::M003_CODEREMOVED_MSG,&[&c_time] ));

                    fs::write(file_path, updated_content.as_bytes())?;                  
                    eprintln!("{}", Messages::format_message(Messages::M005_SUBROUTINE_REMOVED_SUCESS, &[func_name]));
                },
                "all" => {

                    let content = fs::read_to_string(file_path)?;
                    let c_time = Local::now().format("%Y-%m-%d as %H:%M:%S").to_string();
                    
                    let backup_dir = "bkp";
                    fs::create_dir_all(backup_dir)?;
                    let bkpfl_name = format!("{}/{}-{}.f90", backup_dir, c_time, func_name);
                    let bkpfl_path = Path::new(&bkpfl_name);

                    for sub_match in re.find_iter(&content) {
                        let matched_content = &content[sub_match.start()..sub_match.end()]; 
                        fs::write(bkpfl_path, matched_content.as_bytes())?;                    
                    }
                    
                    let updated_content = re.replace_all(&content, &format!(
                        " ! Code removed by sparseFortran application on date {}",
                        c_time
                    )); 

                    let call_pat = format!(r"\bcall\s+{}\b", func_name);
                    let re_call = Regex::new(&call_pat).unwrap();
                    
                    let fim_content = re_call.replace_all(&updated_content, &format_message!(
                        &Messages::M003_CODEREMOVED_MSG,
                        c_time
                    ));
                                         
                    fs::write(file_path, fim_content.as_bytes())?;
                    eprintln!("{}", Messages::format_message(Messages::M002_SUCCESSFUL_REMOVAL_COLORED, &[]));

                },
                _ => {
                    println!("Operação cancelada.");
                }
            }
        } else {
            eprintln!("{}", Messages::format_message(Messages::E004_SUBROUTINE_NOTFOUND, &[func_name, file_path]));

        }
    
        Ok(())
    }   

    fn confirm_removal(file_path: &str, content: &str, target_content: &str) -> io::Result<()> {

        println!("{}",Messages::M001_CONFIRM_REMOVE_MSG);   

        let mut input = String::new();
        io::stdin().read_line(&mut input)?; 
        let input = input.trim().to_lowercase(); 
    
        match input.as_str() {
            "y" => {
                let updated_content = content.replace(target_content, ""); 
                fs::write(file_path, updated_content.as_bytes())?;
                println!("{}",Messages::M002_SUCCESSFUL_REMOVAL_COLORED);


            },
            _ => {
                println!("Operation cancelled.");

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
                eprintln!( "{}",  format_message!(  Messages::E001_METHODFUNCIONORSUBR_NOTFOUND,  self.args.f, self.args.file ) );
                return Err(io::Error::new(io::ErrorKind::NotFound, "Element not found"));

            }
        }
    
        Ok(self)  
    }

    fn show_ascii_art(&self) {
        let buildtype = env::var("PROFILE").unwrap_or_else(|_| "Release".to_string());
        let art = AsciiArt::new(&self.program_name, &buildtype, &self.version);

        let mut color = StandardStream::stdout(ColorChoice::Always);
        color.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();  
        println!("{}", art.render());
        color.reset().unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    fn print_file_info(&self) {
        let data = match fs::metadata(&self.args.file) {
            Ok(meta) => meta,
            Err(_) => {
                eprintln!("\x1b[31m[ error ] Error accessing the file '{}'\x1b[0m", self.args.file);
                return;
            }
        };
    
        let file_name = self.args.file.split('/').last().unwrap_or(&self.args.file);
        let extension = file_name.split('.').last().unwrap_or("Desconhecido");
        let file_size = data.len();
        
        let lines = match fs::read_to_string(&self.args.file) {
            Ok(content) => content.lines().count(),
            Err(_) => {
                eprintln!("\x1b[31m[ error ] Error reading the content of the file '{}'\x1b[0m", self.args.file);
                return; 
            }
        };
    
        println!("\nFile Name: {}", file_name);
        println!("File Path: {}", self.args.file);
        println!("File Extension: {}", extension);
        println!("File Size: {} bytes", file_size);
        println!("Number of Lines: {}\n", lines);
    }    
    
}

fn main() {    
    FacadeApp::new().initialize().unwrap(); 
}
