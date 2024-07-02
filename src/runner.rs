use std::path::Path;

const EXTENSIONS: [&str; 2] = ["exe", "msi"];

pub fn run(prog: String) {
    let path = Path::new(&prog);
    if !path.exists() || !is_valid(&path) {
        println!("Unsupported exectuble");
        return;
    }
    println!("Running program: {:?}", std::fs::canonicalize(path));
}

fn is_valid(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        if let Some(ext) = ext.to_str() {
            return EXTENSIONS.contains(&ext);
        }
    };
    false
}
