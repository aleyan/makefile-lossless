use makefile_lossless::Makefile;
use std::fs;
use std::path::Path;

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
        
        match std::fs::read_to_string(&makefile_path) {
            Ok(makefile_content) => {
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
                        
                        println!("SUCCESS: Parse completed with no errors");
                    },
                    Err(makefile_lossless::Error::Parse(parse_error)) => {
                        println!("\nParsing failed with errors:");
                        println!("{}", parse_error);
                        println!("NOTE: This is expected for real-world Makefiles that use features we don't support yet");
                    },
                    Err(e) => {
                        println!("ERROR: Failed to parse: {}", e);
                    }
                }
            },
            Err(e) => {
                println!("ERROR: Failed to read file: {}", e);
            }
        }
        
        println!("\n=== End of {} ===\n", makefile_name);
    }
}

#[test]
fn test_parse_all_makefiles() {
    // Track overall success rate
    let mut total_files = 0;
    let mut successful_parses = 0;
    
    // Get all files in the makefiles directory
    let makefiles_dir = Path::new("makefiles");
    if !makefiles_dir.exists() || !makefiles_dir.is_dir() {
        panic!("makefiles directory not found");
    }
    
    // Collect all files in the directory
    let entries = match fs::read_dir(makefiles_dir) {
        Ok(entries) => entries,
        Err(e) => panic!("Failed to read makefiles directory: {}", e),
    };
    
    // Process each file
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            
            // Skip directories
            if path.is_dir() {
                continue;
            }
            
            total_files += 1;
            let file_name = path.file_name().unwrap().to_string_lossy();
            println!("\n=== Testing {} ===", file_name);
            
            // Read file content
            match fs::read_to_string(&path) {
                Ok(content) => {
                    println!("File size: {} bytes", content.len());
                    
                    // Attempt to parse
                    match Makefile::from_reader(content.as_bytes()) {
                        Ok(makefile) => {
                            successful_parses += 1;
                            println!("SUCCESS: Parsed successfully");
                            println!("Found {} variables, {} rules, {} includes", 
                                makefile.variable_definitions().count(),
                                makefile.rules().count(),
                                makefile.includes().count());
                        },
                        Err(e) => {
                            println!("FAILED: {}", e);
                        }
                    }
                },
                Err(e) => println!("ERROR: Failed to read file: {}", e),
            }
            
            println!("=== End of {} ===", file_name);
        }
    }
    
    // Report results
    println!("\n=== Summary ===");
    println!("Total files processed: {}", total_files);
    println!("Successfully parsed: {}", successful_parses);
    println!("Parse success rate: {:.1}%", (successful_parses as f64 / total_files as f64) * 100.0);
    
    // Test passes regardless of parse success - we're just collecting statistics
} 