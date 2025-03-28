use makefile_lossless::Makefile;

#[test]
fn test_parse_large_makefile() {
    let makefiles = [
        "Makefile_1",
        "Makefile_2",
        "Makefile_3",
        "Makefile_4",
        "Makefile_5",
        "Makefile_6",
        "Makefile_7",
        "Makefile_8"
    ];

    for makefile_name in makefiles.iter() {
        let makefile_path = std::path::Path::new("makefiles").join(makefile_name);
        println!("\n=== Testing {} ===", makefile_name);
        println!("Attempting to read Makefile from: {:?}", makefile_path);
        
        let makefile_content = std::fs::read_to_string(&makefile_path)
            .expect(&format!("Failed to read {}", makefile_name));
        println!("Successfully read Makefile, content length: {} bytes", makefile_content.len());
        
        // Use from_reader instead of parse() directly to properly handle errors
        match Makefile::from_reader(makefile_content.as_bytes()) {
            Ok(makefile) => {
                println!("\nParsing statistics:");
                
                // Variables
                let vars = makefile.variable_definitions().collect::<Vec<_>>();
                println!("Variable definitions found: {}", vars.len());
                println!("\nFirst few variables:");
                for var in vars.iter().take(5) {
                    if let (Some(name), Some(value)) = (var.name(), var.raw_value()) {
                        println!("{} = {}", name, value);
                    }
                }
                
                // Rules
                let rules = makefile.rules().collect::<Vec<_>>();
                println!("\nRules found: {}", rules.len());
                println!("\nFirst few rules:");
                for rule in rules.iter().take(5) {
                    println!("Targets: {:?}", rule.targets().collect::<Vec<_>>());
                    println!("Prerequisites: {:?}", rule.prerequisites().collect::<Vec<_>>());
                    println!("Recipes: {:?}", rule.recipes().collect::<Vec<_>>());
                    println!();
                }
                
                // Includes
                let includes = makefile.includes().collect::<Vec<_>>();
                println!("\nInclude directives found: {}", includes.len());
                println!("\nFirst few includes:");
                for include in includes.iter().take(5) {
                    println!("Path: {:?}, Optional: {}", include.path(), include.is_optional());
                }
            },
            Err(makefile_lossless::Error::Parse(parse_error)) => {
                println!("\nParsing errors:");
                for error in parse_error.errors.iter() {
                    println!("Error at line {}: {}", error.line, error.message);
                    println!("Context: {}", error.context);
                }
                panic!("Failed to parse {}: had syntax errors", makefile_name);
            },
            Err(e) => {
                panic!("Failed to parse {}: {}", makefile_name, e);
            }
        }
        
        println!("\n=== End of {} ===\n", makefile_name);
    }
} 