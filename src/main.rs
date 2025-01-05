use std::{fs, path::PathBuf, process::exit};
use homedir::unix::my_home;
use toml::Table;

fn main() {

    let xdg_dirs = xdg::BaseDirectories::with_prefix("config-manager").expect("idk how to even get this error");

    let mut path: PathBuf = PathBuf::new();
    // First, make it relative to the current user's home.
    path.push(my_home()
        .expect("no home path found!")
        .unwrap()
        .as_path());
    // Second, find the default values file.
    path.push(".config/config-manager/defaults.toml"
        .to_owned());

    let input_values: String = match fs::read_to_string(&path) {
        Ok(iv) => {
            iv
        }
        Err(_) => {
        
            println!("Not a filepath!");
            exit(2);
        }
    };

    drop(path);

    let templates_path = xdg_dirs
        .place_config_file("templates.toml")
        .expect("cannot create configuration directory");

    let templates: String = match fs::read_to_string(templates_path) {
        Ok(iv) => {
            iv
        }
        Err(_) => {
        
            println!("Not a filepath!");
            exit(2);
        }
    };

    let config_variables = input_values.parse::<Table>().unwrap();
    let templates = templates.parse::<Table>().unwrap();

    for template in templates.values() {

        let mut template_path: PathBuf = PathBuf::new();

        // First, make it relative to the current user's home.
        template_path.push(my_home()
            .expect("no home path found!")
            .unwrap()
            .as_path());

        // Second, find the current template.
        template_path.push(template["input"]
            .as_str()
            .to_owned()
            .unwrap());

        let mut template_string:String = fs::read_to_string(&template_path).expect("Template doesn't exist!");

        for variable in &config_variables {

            let variable_to_replace = format!("{{{}}}", variable.0);

            template_string = template_string.replace(&variable_to_replace, variable.1.as_str().to_owned().unwrap());
        }

        let mut output_path: PathBuf = PathBuf::new();

        // First, make it relative to the current user's home.
        output_path.push(my_home()
            .expect("no home path found!")
            .unwrap()
            .as_path());

        // Second, find the current template.
        output_path.push(template["output"]
        .as_str()
        .to_owned()
        .unwrap());

        println!("{:?}", fs::write(output_path, template_string));
    }
}
