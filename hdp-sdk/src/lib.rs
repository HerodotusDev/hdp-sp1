use hdp_lib::memorizer::Memorizer;
use sp1_sdk::{ExecutionReport, ProverClient, SP1PublicValues, SP1Stdin};
use std::{env, fs, path::Path};
use std::{
    error::Error,
    path::PathBuf,
    process::{Command, Stdio},
};

#[derive(Default, Debug)]
pub struct DataProcessorClient {}

impl DataProcessorClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(
        &self,
        program_path: PathBuf,
    ) -> Result<(SP1PublicValues, ExecutionReport), Box<dyn Error>> {
        // Setup the logger.
        sp1_sdk::utils::setup_logger();

        // Step 1: Run online mode (execute `cargo run -r` in the program directory)
        let status = Command::new("cargo")
            .args(["run", "-r"])
            .current_dir(&program_path)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        if !status.success() {
            return Err(
                format!("Failed to run 'cargo run -r' in {}", program_path.display()).into(),
            );
        };

        // 2. run zkvm mode -> ELF
        let status = Command::new("cargo")
            .args(["prove", "build"])
            .current_dir(&program_path)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        if !status.success() {
            return Err(format!("Failed to build ELF binary in {}", program_path.display()).into());
        }

        // 3. client.execute(ELF, stdin)
        // Setup the prover client.
        let client = ProverClient::new();

        // Setup the inputs.
        let mut stdin = SP1Stdin::new();

        let manifest_dir: String =
            env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
        let path = Path::new(&manifest_dir).join("../memorizer.bin");
        stdin.write(&bincode::deserialize::<Memorizer>(&fs::read(path).unwrap()).unwrap());

        // Execute the program
        let elf_bytes = fs::read("../elf/riscv32im-succinct-zkvm-elf")?;
        let (output, report) = client.execute(&elf_bytes, stdin).run().unwrap();
        println!("Program executed successfully.");
        println!("Number of cycles: {}", report.total_instruction_count());
        Ok((output, report))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        let client = DataProcessorClient::new();
        client.execute("../program".into()).unwrap();
    }
}
