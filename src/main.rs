use std::{collections::{HashMap, HashSet}, path::PathBuf};
use regex::Regex;

const JAVA_PROJECT_PATH: &str = "";
const JAVA_PACKAGE_PATH: &str = "com.softwarearchitecture."; 
fn main() {

    let mut paths = vec![PathBuf::from(JAVA_PROJECT_PATH)];

    let mut classes = HashMap::new();
    let mut packages = HashSet::new();

    while let Some(path) = paths.pop() {
        let entries = std::fs::read_dir(path).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let package_name = path.to_str().unwrap().replace(&format!("{}\\", JAVA_PROJECT_PATH), "").replace('\\', ".");
                // println!("Found directory: {:?}", package_name);
                packages.insert(package_name);
                paths.push(path);
            } else {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                if file_name.ends_with(".java") {
                    let file_path = path.to_str().unwrap().replace(&format!("{}\\", JAVA_PROJECT_PATH), "").replace('\\', "/");
                    let file_contents = std::fs::read_to_string(&path).unwrap();
                    // println!("Found java file: {:?}", file_path);
                    classes.insert(file_path, file_contents);
                }
            }
        }
    }

    // Dependencies between classes
    let mut dependencies: HashMap<String, HashSet<String>> = HashMap::new();
    for (file_path, file_contents) in classes.iter() {
        let class_name = file_path.replace('/', ".").replace(".java", "");
        let mut dependencies_hash_set = HashSet::new();
        for line in file_contents.lines() {
            if line.contains("import") && line.contains(JAVA_PACKAGE_PATH) {
                let dependency = line.replace("import ", "").replace("com.softwarearchitecture.", "").replace(';', "");
                // let dependency = line.replace("import ", "").replace(";", "").replace("static ", "").replace(".*", "");
                dependencies_hash_set.insert(dependency);
            }
        }
        // Check for all dependencies withing the exact same package
        let current_package = class_name.split('.').take(class_name.split('.').count() - 1).collect::<Vec<&str>>().join(".");
        for (dependency_file_path, _) in classes.iter() {
            let dependency_class_name = dependency_file_path.replace('/', ".").replace(".java", "");
            let dependency_class_package = dependency_class_name.split('.').take(dependency_class_name.split('.').count() - 1).collect::<Vec<&str>>().join(".");
            let dependency_class_name_short = dependency_class_name.split('.').last().unwrap();
            if current_package == dependency_class_package && file_contents.contains(dependency_class_name_short) && class_name != dependency_class_name {
                dependencies_hash_set.insert(dependency_class_name);
            }
        }

        dependencies.insert(class_name, dependencies_hash_set);
    }

    // Convert classes to plant uml
    let mut plant_uml_classes: HashMap<String, String> = HashMap::new();
    
    add_file_data("public class", r"public\s+(static\s+)?(final\s+)?([\w\[\]<>]+[\s\*]+)+(\w+)\s*\(([^)]+)\)(\s*throws\s+[^{]+)?\s*\{", "class", &classes, &mut plant_uml_classes);
    // for (file_path, file_contents) in classes.iter() {
    //     if !file_contents.contains("public class") {
    //         continue;
    //     }
    //     let class_name = file_path.replace('/', ".").replace(".java", "");
    //     let mut functions = HashMap::new();
    //     // println!("Class name: {:?}", class_name);

    //     let mut no_double_spaces = file_contents.replace('\t', "");
    //     while no_double_spaces.contains("  ") {
    //         no_double_spaces = no_double_spaces.replace("  ", " ");
    //     }
    //     let re = Regex::new(r"public\s+(static\s+)?(final\s+)?([\w\[\]<>]+[\s\*]+)+(\w+)\s*\(([^)]+)\)(\s*throws\s+[^{]+)?\s*\{").unwrap();
    //     for cap in re.captures_iter(&no_double_spaces.replace("\r\n", "").replace('\n', "")) {
    //         let function_name = cap.get(4).unwrap().as_str();
    //         let function_args = cap.get(5).unwrap().as_str();
    //         let function_return_type = cap.get(3).unwrap().as_str();
    //         // println!("Function name: {:?}, args: {:?}", function_name, function_args);
    //         functions.insert(function_name.to_string(), (function_args.to_string(), function_return_type.to_string()));
    //     }

    //     let plant_uml_class = String::from("\nclass CLASS_NAME_MARKER as CLASS_FINDER_MARKER {\nFUNCTION_MARKER}\n");
    //     let mut functions_str = String::new();
    //     for (function_name, (function_args, function_return)) in functions.iter() {
    //         functions_str.push_str(&format!("\t+{}({}): {}\n", function_name, function_args, function_return));
    //     }

    //     let mut class_finder = String::new();
    //     if class_name.contains('.') {
    //         class_finder += "as ";
    //         class_finder += &class_name.replace('.', "");
    //     }
    //     plant_uml_classes.insert(class_name.to_string(), plant_uml_class.replace("CLASS_NAME_MARKER", class_name.split('.').last().unwrap()).replace("FUNCTION_MARKER", &functions_str).replace("as CLASS_FINDER_MARKER", &class_finder.replace('_', "")));
    //     // println!("{} {:?}", class_name, functions);
    // }

    // Convert abstract classes to plant uml
    add_file_data("public abstract class", r"public\s+abstract\s+class\s+\w+\s*[^{]*\{(?:[^{}]*\{[^{}]*\}[^{}]*)*?\s*public\s+(static\s+)?(final\s+)?([\w\[\]<>]+[\s\*]+)+(\w+)\s*\((.*?)\)(\s*throws\s+[^{]+)?\s*\{", "abstract class", &classes, &mut plant_uml_classes);
    // for (file_path, file_contents) in classes.iter() {
    //     if !file_contents.contains("public abstract class") {
    //         continue;
    //     }
    //     let class_name = file_path.replace('/', ".").replace(".java", "");
    //     let mut functions = HashMap::new();
    //     // println!("Class name: {:?}", class_name);

    //     let mut no_double_spaces = file_contents.replace('\t', "");
    //     while no_double_spaces.contains("  ") {
    //         no_double_spaces = no_double_spaces.replace("  ", " ");
    //     }
    //     let re = Regex::new(r"public\s+abstract\s+class\s+\w+\s*[^{]*\{(?:[^{}]*\{[^{}]*\}[^{}]*)*?\s*public\s+(static\s+)?(final\s+)?([\w\[\]<>]+[\s\*]+)+(\w+)\s*\((.*?)\)(\s*throws\s+[^{]+)?\s*\{").unwrap();
    //     for cap in re.captures_iter(&no_double_spaces.replace("\r\n", "").replace('\n', "")) {
    //         let function_name = cap.get(4).unwrap().as_str();
    //         let function_args = cap.get(5).unwrap().as_str();
    //         let function_return_type = cap.get(3).unwrap().as_str();
    //         // println!("Function name: {:?}, args: {:?}, return: {:?}", function_name, function_args, function_return_type);
    //         functions.insert(function_name.to_string(), (function_args.to_string(), function_return_type.to_string()));
    //     }

    //     let plant_uml_class = String::from("\nabstract class CLASS_NAME_MARKER as CLASS_FINDER_MARKER {\nFUNCTION_MARKER}\n");
    //     let mut functions_str = String::new();
    //     for (function_name, (function_args, function_return)) in functions.iter() {
    //         functions_str.push_str(&format!("\t+{}({}): {}\n", function_name, function_args, function_return));
    //     }

    //     let mut class_finder = String::new();
    //     if class_name.contains('.') {
    //         class_finder += "as ";
    //         class_finder += &class_name.replace('.', "");
    //     }
    //     plant_uml_classes.insert(class_name.to_string(), plant_uml_class.replace("CLASS_NAME_MARKER", class_name.split('.').last().unwrap()).replace("FUNCTION_MARKER", &functions_str).replace("as CLASS_FINDER_MARKER", &class_finder));
    // }

    // Convert interfaces to plant uml
    add_file_data("public interface", r"public\s+interface\s+\w+\s*\{[^{}]*?\s*((?:static\s+)?(?:final\s+)?)(([\w\[\]<>]+[\s\*]*)+)\s+(\w+)\s*\((.*?)\)(\s*throws\s+[^{]+)?\s*;", "interface", &classes, &mut plant_uml_classes);
    // for (file_path, file_contents) in classes.iter() {
    //     if !file_contents.contains("public interface") {
    //         continue;
    //     }
    //     let class_name = file_path.replace('/', ".").replace(".java", "");
    //     let mut functions = HashMap::new();
    //     // println!("Class name: {:?}", class_name);

    //     let mut no_double_spaces = file_contents.replace('\t', "");
    //     while no_double_spaces.contains("  ") {
    //         no_double_spaces = no_double_spaces.replace("  ", " ");
    //     }
    //     let re = Regex::new(r"public\s+interface\s+\w+\s*\{[^{}]*?\s*((?:static\s+)?(?:final\s+)?)(([\w\[\]<>]+[\s\*]*)+)\s+(\w+)\s*\((.*?)\)(\s*throws\s+[^{]+)?\s*;").unwrap();
    //     for cap in re.captures_iter(&no_double_spaces.replace("\r\n", "").replace('\n', "")) {
    //         let function_name = cap.get(4).unwrap().as_str();
    //         let function_args = cap.get(5).unwrap().as_str();
    //         let function_return_type = cap.get(3).unwrap().as_str();
    //         // println!("Function name: {:?}, args: {:?}, return: {:?}", function_name, function_args, function_return_type);
    //         functions.insert(function_name.to_string(), (function_args.to_string(), function_return_type.to_string()));
    //     }

    //     let plant_uml_class = String::from("\ninterface CLASS_NAME_MARKER as CLASS_FINDER_MARKER {\nFUNCTION_MARKER}\n");
    //     let mut functions_str = String::new();
    //     for (function_name, (function_args, function_return)) in functions.iter() {
    //         functions_str.push_str(&format!("\t+{}({}): {}\n", function_name, function_args, function_return));
    //     }

    //     let mut class_finder = String::new();
    //     if class_name.contains('.') {
    //         class_finder += "as ";
    //         class_finder += &class_name.replace('.', "");
    //     }
    //     plant_uml_classes.insert(class_name.to_string(), plant_uml_class.replace("CLASS_NAME_MARKER", class_name.split('.').last().unwrap()).replace("FUNCTION_MARKER", &functions_str).replace("as CLASS_FINDER_MARKER", &class_finder));
    // }

    // Write to file
    let mut package_definitions: HashMap<String, String> = HashMap::new();
    for (package_name, plant_uml_class) in plant_uml_classes.iter() {
        let package_name = package_name.split('.').take(package_name.split('.').count() - 1).collect::<Vec<&str>>().join(".");
        if package_name.is_empty() {
            // no_package_classes.insert(plant_uml_class);
            continue;
        }
        let package_definition_new = String::from("package PACKAGE_NAME_MARKER {\n");
        let package_definition = package_definitions.entry(package_name.clone()).or_insert(package_definition_new.replace("PACKAGE_NAME_MARKER", &package_name));
        package_definition.push_str(plant_uml_class.lines().map(|line| format!("\t{}\n", line)).collect::<String>().as_str());
    }

    for package in packages.iter().map(|package| package.split('.').next().unwrap().to_string()).collect::<HashSet<String>>().iter() {
        let mut plant_uml = String::new();
        
        // Write all classes in this package and its subpackages and put them in their respective packages
        for (package_name, package_definition) in package_definitions.iter() {
            if package_name.starts_with(package) {
                plant_uml.push_str(format!("{}}}\n", package_definition).as_str());
            }
        }

        // Find all dependencies for the classes in this package and its subpackages and add them to the plant uml by adding their classes and packages
        let mut dependencies_in_package = HashMap::new();
        for (class_name, dependencies_vec) in dependencies.iter() {
            if class_name.starts_with(package) {
                dependencies_in_package.insert(class_name, dependencies_vec);
            }
        }
        let mut external_classes_that_are_depended_on = HashSet::new();
        for (_, dependencies_vec) in dependencies_in_package.iter() {
            for dependency in dependencies_vec.iter() {
                if !dependency.starts_with(package) {
                    external_classes_that_are_depended_on.insert(dependency);
                }
            }
        }

        dbg!(package, &external_classes_that_are_depended_on);
        let mut depended_package_definitions: HashMap<String, String> = HashMap::new();
        for (package_name, plant_uml_class) in plant_uml_classes.iter() {
            let class_name = package_name.replace('\\', ".");
            let package_name = package_name.split('.').take(package_name.split('.').count() - 1).collect::<Vec<&str>>().join(".");
            if package_name.is_empty() || !external_classes_that_are_depended_on.contains(&class_name) {
                // no_package_classes.insert(plant_uml_class);
                continue;
            }
            println!("Package name: {:?}", class_name);
            let package_definition_new = String::from("package PACKAGE_NAME_MARKER {\n");
            let package_definition = depended_package_definitions.entry(package_name.clone()).or_insert(package_definition_new.replace("PACKAGE_NAME_MARKER", &package_name));
            package_definition.push_str(plant_uml_class.lines().map(|line| format!("\t{}\n", line)).collect::<String>().as_str());
        }
        
        for (_, package_definition) in depended_package_definitions.iter() {
            plant_uml.push_str(format!("{}}}\n", package_definition).as_str());
        }

        // let mut visited_packages = HashSet::new();
        // for external_package in external_packages_that_are_depended_on.iter() {
        //     let external_package_name = external_package.split('.').take(external_package.split('.').count() - 1).collect::<Vec<&str>>().join(".");
        //     for (package_name, package_definition) in package_definitions.iter() {
        //         if *package_name == external_package_name {
        //             if visited_packages.contains(external_package_name.as_str()) {
        //                 continue;
        //             }
        //             visited_packages.insert(external_package_name.clone());
        //             plant_uml.push_str(format!("{}}}\n", package_definition).as_str());
        //         }
        //     }
        // }
        
        for (class_name, dependencies_vec) in dependencies_in_package.iter() {
            for dependency in dependencies_vec.iter() {
                plant_uml.push_str(&format!("{} --> {}\n", class_name.replace('.', ""), dependency.replace('.', "")));
            }
        }
        
        write_plant_uml_data_to_file(&format!("{}.puml", package.replace('.', "_")), &plant_uml)
    }


    // let mut no_package_classes = HashSet::new();
    //     let mut package_definitions: HashMap<String, String> = HashMap::new();
    //     for (package_name, plant_uml_class) in plant_uml_classes.iter() {
    //         let package_name = package_name.split('.').take(package_name.split('.').count() - 1).collect::<Vec<&str>>().join(".");
    //         if package_name.is_empty() {
    //             no_package_classes.insert(plant_uml_class);
    //             continue;
    //         }
    //         let package_definition_new = String::from("package PACKAGE_NAME_MARKER {\n");
    //         let package_definition = package_definitions.entry(package_name.clone()).or_insert(package_definition_new.replace("PACKAGE_NAME_MARKER", &package_name));
    //         package_definition.push_str(plant_uml_class.lines().map(|line| format!("\t{}\n", line)).collect::<String>().as_str());
    //     }
    
    //     for no_package_class in no_package_classes.iter() {
    //         plant_uml.push_str(no_package_class);
    //     }
    
    //     for (_, package_definition) in package_definitions.iter() {
    //         plant_uml.push_str(format!("{}}}\n", package_definition).as_str());
    //     }
    
    //     for (class_name, dependencies_vec) in dependencies.iter() {
    //         for dependency in dependencies_vec.iter() {
    //             plant_uml.push_str(&format!("{} --> {}\n", class_name.replace('.', ""), dependency.replace('.', "")));
    //         }
    //     }
}

fn add_file_data(class_type: &str, regex_text: &str, plant_uml_prefix: &str, classes: &HashMap<String, String>, plant_uml_classes: &mut HashMap<String, String>) {
    for (file_path, file_contents) in classes.iter() {
        if !file_contents.contains(class_type) {
            continue;
        }
        let class_name = file_path.replace('/', ".").replace(".java", "");
        let mut functions = HashMap::new();
        // println!("Class name: {:?}", class_name);

        let mut no_double_spaces = file_contents.replace('\t', "");
        while no_double_spaces.contains("  ") {
            no_double_spaces = no_double_spaces.replace("  ", " ");
        }
        let re = Regex::new(regex_text).unwrap();
        for cap in re.captures_iter(&no_double_spaces.replace("\r\n", "").replace('\n', "")) {
            let function_name = cap.get(4).unwrap().as_str();
            let function_args = cap.get(5).unwrap().as_str();
            let function_return_type = cap.get(3).unwrap().as_str();
            // println!("Function name: {:?}, args: {:?}", function_name, function_args);
            functions.insert(function_name.to_string(), (function_args.to_string(), function_return_type.to_string()));
        }

        let plant_uml_class = format!("\n{} \"CLASS_NAME_MARKER\" as CLASS_FINDER_MARKER {{\nFUNCTION_MARKER}}\n", plant_uml_prefix);
        let mut functions_str = String::new();
        for (function_name, (function_args, function_return)) in functions.iter() {
            functions_str.push_str(&format!("\t+{}({}): {}\n", function_name, function_args, function_return));
        }

        let mut class_finder = String::new();
        if class_name.contains('.') {
            class_finder += "as ";
            class_finder += &class_name.replace('.', "");
        }
        plant_uml_classes.insert(class_name.to_string(), plant_uml_class.replace("CLASS_NAME_MARKER", class_name.split('.').last().unwrap()).replace("FUNCTION_MARKER", &functions_str).replace("as CLASS_FINDER_MARKER", &class_finder));
    }
}


fn write_plant_uml_data_to_file(file_name: &str, plant_uml_data: &String) {
    let mut data = String::from("@startuml\n");
    data += plant_uml_data;
    data += "@enduml";

    std::fs::write(file_name, data).unwrap();
}