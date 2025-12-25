use std::{fs};
use toml::{Table};
use std::process::Command;

fn main() {

    let args: Vec<String> = std::env::args().collect();
    if (args.len()-1) < 1 {
        println!("[USAGE]");
        println!("<program> [theme]");
        println!();
        std::process::exit(1);
    }

    let theme: &str = &args[1];


    let config_path: String = dirs::config_dir().unwrap().into_os_string().into_string().unwrap()
    + "/config-manager/";
    let home_path: String = dirs::home_dir().unwrap().into_os_string().into_string().unwrap()
    + "/";


    let input_values: String = match fs::read_to_string(config_path.clone() + "themes/" + theme +".toml") {
        Ok(data) => data,
        Err(_) => {
            println!("No {} theme file found!", theme);
            return;
        }
    };

    let templates: String = match fs::read_to_string(config_path.clone() + "templates.toml") {
        Ok(data) => data,
        Err(_) => {
            println!("No templates file found!");
            return;
        }
    };

    let config_variables = input_values.parse::<Table>().unwrap();
    let templates = templates.parse::<Table>().unwrap();

    for template in templates.values() {

        if template.get("input") != None {

            let template_path: &str = &template["input"].as_str().unwrap();

            let mut template_string:String = match fs::read_to_string(home_path.clone() + template_path) {
                Ok(data) => data,
                Err(_) => {
                    println!("Template {} doesn't exist!", template["input"]);
                    return;
                }
            };
    
            for variable in &config_variables {
    
                let variable_to_replace = format!("{{{{{}}}}}", variable.0);
    
                template_string = template_string.replace(&variable_to_replace, variable.1.as_str().to_owned().unwrap());
            }
    
            let output_path: &str = &template["output"].as_str().unwrap();
            
            //println!("{}", home_path.clone() + output_path);
    
            match fs::write(home_path.clone() + output_path, template_string) {
                Ok(data) => data,
                Err(_error) => {
                    println!("File failed to write!");
                    return;
                } 
            };
        }

        if template.get("hook") != None {
            Command::new("bash")
                .arg("-c")
                .arg(template["hook"].as_str().to_owned().unwrap())
                .output()
                .expect("failed to execute process").stdout;
        }
    }
}
