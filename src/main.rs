extern crate markdown;



fn main() {
    
    let header_template = read_file_to_string("templates/header.html");
    let footer_template = read_file_to_string("templates/footer.html");
    
    let input_read = get_input_read();
    
    for input_entry in input_read
    {
        let entry = input_entry.unwrap();
       
        if entry.path().extension().unwrap() == "md"
        {
            generate_from_markdown(&entry, &header_template, &footer_template);
        }
    }
}

fn get_input_read() -> std::fs::ReadDir
{
    let paths = match std::fs::read_dir("input") {
        Ok(x) => x,
        Err(_) => {
            match std::fs::create_dir("input")
            {
                Ok(_) => {
                    panic!("Created directory input rerun the program with actual markdown files in input!");
                }
                Err(error) => {
                    panic!("Failed to create input folder with error {error}!");
                }
            }
        },
    };
    return paths;
}

fn generate_from_markdown(entry: &std::fs::DirEntry, header_template: &str, footer_template: &str)
{
    let markdown = read_file_to_string(entry.path().to_str().unwrap());
    
    let options = comrak::ComrakOptions::default();
    let html : String = comrak::markdown_to_html(markdown.as_str(),&options);
    
    let combined : String = header_template.to_string() + html.as_str() + footer_template;
    
    
    let path_prefix = "output/";
    let file_path = path_prefix.to_string() + entry.path().file_stem().unwrap().to_str().unwrap() + ".html"; 
    
    
    let path = std::path::Path::new(path_prefix);
    
    if !path.exists()
    {
        match std::fs::create_dir(path) {
            Ok(_) => {}
            Err(error) => {
                panic!("Failed to create directory {} with error {}", path_prefix, error);
            }
        };
    }
    let _ = match std::fs::write(std::path::Path::new(file_path.as_str()), combined)
    {
        Ok(_) => { }
        Err(error) => {
            
            panic!("Failed to write file {} with error {}", entry.path().display(), error);
        }
    };
}

fn read_file_to_string(path : &str) -> String
{
    let contents = match std::fs::read_to_string(std::path::Path::new(path))
    {
        Ok(test_contents) => {test_contents}
        Err(error) => { panic!("Couldn't read file with path {} to string! Error code: {path}", error); }
    };

    return contents;
}

// fn print_current_dir()
// {
//     let a: std::path::PathBuf = match std::env::current_dir() {
//         Ok(a) => a,
//         Err(error) => { panic!("Path was not found with error {error}")},
//     };
// 
//     print!("{}", a.display());
// }