use hdp_lib::memorizer::Memorizer;
use hdp_lib::utils::find_workspace_root;
use sp1_sdk::{
    ExecutionReport, ProverClient, SP1ProofWithPublicValues, SP1PublicValues, SP1Stdin,
    SP1VerifyingKey,
};
use std::fmt::Debug;
use std::{env, fs};
use std::{
    error::Error,
    path::PathBuf,
    process::{Command, Stdio},
};

#[derive(Default)]
pub struct DataProcessorClient {
    pub sp1_client: ProverClient,
}

impl Debug for DataProcessorClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataProcessorClient").finish()
    }
}

impl DataProcessorClient {
    pub fn new() -> Self {
        Self {
            sp1_client: ProverClient::new(),
        }
    }

    pub fn execute(
        &self,
        program_path: PathBuf,
    ) -> Result<(SP1PublicValues, ExecutionReport), Box<dyn Error>> {
        // Setup the logger.
        env::set_var("RUST_LOG", "info");
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

        // Setup the inputs.
        let mut stdin = SP1Stdin::new();
        let workspace_root = find_workspace_root().expect("Workspace root not found");
        let path = workspace_root.join("memorizer.bin");
        println!("Memorizer loaded from {path:?}");
        stdin.write(&bincode::deserialize::<Memorizer>(&fs::read(path).unwrap()).unwrap());

        // ELF
        let path = workspace_root.join("elf/riscv32im-succinct-zkvm-elf");
        println!("ELF loaded from {path:?}");
        let elf_bytes = fs::read(path)?;

        // Execute the program
        let (output, report) = self.sp1_client.execute(&elf_bytes, stdin).run().unwrap();
        println!("Program executed successfully.");
        println!("Number of cycles: {}", report.total_instruction_count());
        Ok((output, report))
    }

    pub fn prove(
        &self,
        program_path: PathBuf,
    ) -> Result<(SP1ProofWithPublicValues, SP1VerifyingKey), Box<dyn Error>> {
        // Setup the logger.
        env::set_var("RUST_LOG", "info");
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

        // Setup the inputs.
        let mut stdin = SP1Stdin::new();
        let workspace_root = find_workspace_root().expect("Workspace root not found");
        let path = workspace_root.join("memorizer.bin");
        println!("Memorizer loaded from {path:?}");
        stdin.write(&bincode::deserialize::<Memorizer>(&fs::read(path).unwrap()).unwrap());

        // ELF
        let path = workspace_root.join("elf/riscv32im-succinct-zkvm-elf");
        println!("ELF loaded from {path:?}");
        let elf_bytes = fs::read(path)?;

        // Setup the program for proving.
        let (pk, vk) = self.sp1_client.setup(&elf_bytes);

        // Generate the proof
        let proof = self
            .sp1_client
            .prove(&pk, stdin)
            .run()
            .expect("failed to generate proof");

        Ok((proof, vk))
    }

    pub fn verify(
        &self,
        proof: &SP1ProofWithPublicValues,
        vk: &SP1VerifyingKey,
    ) -> Result<(), Box<dyn Error>> {
        // Setup the logger.
        env::set_var("RUST_LOG", "info");
        sp1_sdk::utils::setup_logger();
        // Verify the proof.
        self.sp1_client
            .verify(proof, vk)
            .expect("failed to verify proof");
        println!("Successfully verified proof!");
        Ok(())
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

    #[test]
    fn test_verify() {
        let client = DataProcessorClient::new();
        let (proof, vk) = client.prove("../program".into()).unwrap();
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}