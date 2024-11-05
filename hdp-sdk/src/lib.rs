use hdp_lib::memorizer::Memorizer;
use hdp_lib::utils::find_workspace_root;
use sp1_sdk::{
    ExecutionReport, NetworkProver, ProverClient, SP1ProofWithPublicValues, SP1PublicValues,
    SP1Stdin, SP1VerifyingKey,
};
use std::fmt::Debug;
use std::io::Write;
use std::{env, fs};
use std::{
    error::Error,
    path::PathBuf,
    process::{Command, Stdio},
};

#[derive(Default)]
pub struct DataProcessorClient {
    pub sp1_client: ProverClient,
    pub inputs: Vec<Box<dyn erased_serde::Serialize>>,
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
            inputs: Vec::new(),
        }
    }

    pub fn write<T>(&mut self, value: T)
    where
        T: serde::Serialize + 'static,
    {
        self.inputs.push(Box::new(value));
    }

    pub fn execute(
        &mut self,
        program_path: PathBuf,
    ) -> Result<(SP1PublicValues, ExecutionReport), Box<dyn Error>> {
        let (elf_bytes, stdin) = self.setup(program_path)?;

        // Execute the program
        let (output, report) = self
            .sp1_client
            .execute(&elf_bytes, stdin)
            .run()
            .expect("failed to execute program");
        println!("Program executed successfully.");
        println!("Number of cycles: {}", report.total_instruction_count());
        Ok((output, report))
    }

    pub async fn network_prove(
        &self,
        program_path: PathBuf,
        private_key: String,
    ) -> Result<(String, NetworkProver), Box<dyn Error>> {
        let (elf_bytes, stdin) = self.setup(program_path)?;

        let network_client = sp1_sdk::NetworkProver::new_from_key(&private_key);
        let proof = network_client
            .request_proof(
                &elf_bytes,
                stdin,
                sp1_sdk::proto::network::ProofMode::Groth16,
            )
            .await
            .unwrap();
        Ok((proof, network_client))
    }

    pub fn prove(
        &self,
        program_path: PathBuf,
    ) -> Result<(SP1ProofWithPublicValues, SP1VerifyingKey), Box<dyn Error>> {
        let (elf_bytes, stdin) = self.setup(program_path)?;

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

    fn setup(&self, program_path: PathBuf) -> Result<(Vec<u8>, SP1Stdin), Box<dyn Error>> {
        // Setup the logger.
        env::set_var("RUST_LOG", "info");
        sp1_sdk::utils::setup_logger();

        // Step 1: Run online mode (execute `cargo run -r` in the program directory)
        let mut child = Command::new("cargo")
            .args(["run", "-r"])
            .current_dir(&program_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            for input in &self.inputs {
                let serialized_data = bincode::serialize(&input)?;
                stdin.write_all(&serialized_data)?;
            }
        }

        if !child.wait()?.success() {
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
        let mem = &bincode::deserialize::<Memorizer>(
            &fs::read(path).expect("Failed to read memorizer.bin"),
        )
        .expect("Failed to deserialize memorizer.bin");
        // TODO: probably will need to use this value to config verifier contract chain environment.
        let _to_chain_id = mem.to_chain_id;
        stdin.write(mem);
        for input in &self.inputs {
            stdin.write(&input);
        }

        // ELF
        let path = workspace_root.join("elf/riscv32im-succinct-zkvm-elf");
        println!("ELF loaded from {path:?}");
        let elf_bytes = fs::read(path)?;

        Ok((elf_bytes, stdin))
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
        let mut client = DataProcessorClient::new();
        // client.write(5244652_u64);
        // client.write(11155111_u64);
        client.execute("../program".into()).unwrap();
    }

    #[test]
    fn test_verify() {
        let mut client = DataProcessorClient::new();
        client.write(5244652_u64);
        client.write(11155111_u64);
        let (proof, vk) = client.prove("../program".into()).unwrap();
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }

    #[tokio::test]
    async fn test_verify_network() {
        // let key = env::var("SP1_PRIVATE_KEY").unwrap();
        let mut client = DataProcessorClient::new();
        client.write(5244652_u64);
        client.write(11155111_u64);
        let (proof_key, network_client) = client
            .network_prove(
                "../program".into(),
                "0x8f6f1610b5b22088fed7eb1d8c0ab3abfd5433bfcfb8b0a464b67bdaa3cabc10".into(),
            )
            .await
            .unwrap();
        let res: SP1ProofWithPublicValues =
            network_client.wait_proof(&proof_key, None).await.unwrap();
        // Save the proof.
        res.save("hdp-sp1-groth16.bin")
            .expect("saving proof failed");
        println!("Successfully verified proof!");
    }
}
