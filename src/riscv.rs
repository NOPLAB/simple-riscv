pub struct Emulator {
    path: String
}

impl Emulator {
    pub fn init(path: String) -> Self {
        Self {
            path
        }
    }

    pub fn run(self) {
        println!("RUN - {path}");
    }
}

