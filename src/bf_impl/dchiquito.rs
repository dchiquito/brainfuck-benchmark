use crate::*;

#[derive(Clone)]
pub struct DchiquitoImpl(String);

lazy_static::lazy_static! {
    /// Source Website
    static ref WEBSITE: String = String::from("https://github.com/dchiquito/bf-compiler");
    /// Source URL
    static ref URL: String = String::from("https://github.com/dchiquito/bf-compiler.git");
}

impl DchiquitoImpl {
    pub fn new(name: &str) -> DchiquitoImpl {
        DchiquitoImpl(name.to_string())
    }
    pub fn src_dir(&self) -> String {
        format!("build/src/dchiquito/{}", self.0)
    }
    pub fn result_exe(&self) -> String {
        format!("build/src/dchiquito/{}/target/release/bf-compiler", self.0)
    }
}

impl BFImpl for DchiquitoImpl {
    fn name(&self) -> String {
        format!("dchiquito/{}", self.0)
    }

    fn interpreted(&self) -> bool {
        true
    }

    fn enabled(&self) -> bool {
        true
    }

    fn get(&self) {
        git_repo_branch(&URL, &self.src_dir(), &self.0);
    }

    fn build(&self) {
        run_command(
            Command::new("cargo")
                .args(["build", "--release"])
                .current_dir(self.src_dir()),
        );
    }

    fn prepare(&self, _file: PathBuf) {}

    fn get_invoke_command(&self, file: PathBuf) -> String {
        let file_str = file.to_string_lossy().to_string();
        format!("{} {}", self.result_exe(), file_str)
    }

    fn filter_output(&self, contents: String) -> String {
        let regex = regex::Regex::new(&format!("`{}.*?`", self.result_exe())).unwrap();
        regex
            .replace(
                &contents,
                format!("[`{}`]({})", self.name(), &*WEBSITE).as_str(),
            )
            .into()
    }
}
