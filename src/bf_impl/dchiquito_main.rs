use crate::*;

#[derive(Clone)]
pub struct DchiquitoMainImpl;

lazy_static::lazy_static! {
    /// Name of the interpreter. Often a github repo or website name.
    static ref NAME: String = String::from("dchiquito/main");
    /// Source Website
    static ref WEBSITE: String = String::from("https://github.com/dchiquito/bf-compiler");
    /// Source URL
    static ref URL: String = String::from("https://github.com/dchiquito/bf-compiler.git");
    /// Source folder
    static ref SRC_DIR: String = String::from("build/src/dchiquito/main");
    /// Actual EXE ran.
    static ref RESULT_EXE: String = String::from("build/src/dchiquito/main/target/release/bf-compiler");
}

impl BFImpl for DchiquitoMainImpl {
    fn name(&self) -> String {
        NAME.clone()
    }

    fn interpreted(&self) -> bool {
        true
    }

    fn enabled(&self) -> bool {
        true
    }

    fn get(&self) {
        git_repo_branch(&URL, &SRC_DIR, "main");
    }

    fn build(&self) {
        run_command(
            Command::new("cargo")
                .args(["build", "--release"])
                .current_dir(&*SRC_DIR),
        );
    }

    fn prepare(&self, _file: PathBuf) {}

    fn get_invoke_command(&self, file: PathBuf) -> String {
        let file_str = file.to_string_lossy().to_string();
        format!("{} {}", &*RESULT_EXE, file_str)
    }

    fn filter_output(&self, contents: String) -> String {
        let regex = regex::Regex::new(&format!("`{}.*?`", &*RESULT_EXE)).unwrap();
        regex
            .replace(&contents, format!("[`{}`]({})", &*NAME, &*WEBSITE).as_str())
            .into()
    }
}
