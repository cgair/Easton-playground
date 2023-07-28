//! The builder pattern

// In Rust, there are two variants of the builder pattern, differing in the treatment of ownership.

pub struct Process;

/// Non-consuming builders (preferred)
// NOTE: the actual Command API does not use owned Strings;
// this is a simplified version.
pub struct Command {
    program: String,
    args: Vec<String>,
    cwd: Option<String>,
    // etc.
}

impl Command {
    pub fn new(program: &str) -> Self {
        Command {
            program: program.to_string(),
            args: Vec::new(),
            cwd: None
        }
    }

    /// Add an argument to pass to the program
    pub fn arg(&mut self, arg: &str) -> &mut Self {
        self.args.push(arg.to_string());
        self
    }

    pub fn args(&mut self, args: &[String]) -> &mut Self {
        self.args.extend(args.to_vec());
        self
    }

    /// Set the working directory for the child process.
    pub fn cwd<'a>(&'a mut self, dir: String) -> &'a mut Command {
        self.cwd = Some(dir);
        self
    }

    /// Executes the command as a child process, which is returned.
    pub fn spawn(&self) -> std::io::Result<Process> {   // spawn method actually uses the builder configuration to spawn a process, takes the builder by immutable reference.
        // snip ..                                      //  This is possible because spawning the process does not require ownership of the configuration data.
        Ok(Process)
    }
}

// Consuming builders
// [ThreadBuilder](https://doc.rust-lang.org/src/std/thread/mod.rs.html#275-280)


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_non_consuming() {
        // The benefit: using borrows throughout, `Command` can be used conveniently for both one-liner and more complex constructions

        // One-liners
        let _ = Command::new("/bin/cat").arg("file.txt").spawn().unwrap();

        // Complex configuration
        let size_sorted = true;
        let mut cmd = Command::new("/bin/ls");
        cmd.arg(".");

        if size_sorted {
            cmd.arg("-S");
        }

        cmd.spawn().unwrap();
    }

    use std::thread::Builder as ThreadBuilder;
    #[test]
    fn it_consuming() {
        // One-liners
        // One-liners work as before, because ownership is threaded through each of the builder methods until being consumed by spawn.
        let _thread = ThreadBuilder::new().name("consuming".to_string()).stack_size(2048).spawn(|| { }).unwrap();     

        // Complex configuration
        // Complex configuration, however, is more verbose: it requires re-assigning the builder at each step.
        let mut thread = ThreadBuilder::new();
        thread = thread.name("consuming2".to_string());
        thread = thread.stack_size(4096);
        thread.spawn(|| {}).unwrap();
    }
}

