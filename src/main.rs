fn main() {

    let info = wix::structs::Information {
        name: "wix".to_string(),
        author: "miten".to_string(),
        version: "0.1.0".to_string(),
        description: "wix - cross platform package manager".to_string(),
        license: "zlib".to_string(),
        repository: "https://github.com/m1ten/wix".to_string()
    };

    let args = wix::args::Arguments::new(info.clone());

    println!("Wix!\n");

    if wix::setup::is_super() {
        eprintln!("{}", "Error: You are running wix as root.");
        wix::exit!(1);
    }

    if !wix::setup::is_python_installed() {
        eprintln!("Error: Python >=3.8 is not installed.");
        eprintln!("Please install and add Python to path then try again.");
        wix::exit!(127);
    }

    // wix::setup::run(info, args);

    // match std::env::var("USER") {
    //     Ok(user) => { 
    //         match user.as_str() {
    //             "root" => { eprintln!("Please run wix as a regular user."); return },
    //             _ => (),
    //         }
    //     },
    //     Err(e) => ()
    // }

    // let info_contents = wix::structs::Information::get_field_type(Some(info));

    // let mut info_code = wix::lang::struct_to_py("Information".to_string(), info_contents);
    // info_code = info_code.replace("Information = {}", "").replace("Information.", "");

    // wix::file::writefs(
    //     "wix.py".to_string(),
    //     info_code.trim_start().to_string().trim_end().to_string()
    // ).unwrap();
}   