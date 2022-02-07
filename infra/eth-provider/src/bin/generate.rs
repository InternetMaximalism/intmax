// grcov: ignore-start
use convert_case::{Case, Casing};

use ethcontract_generate::loaders::HardHatLoader;
use ethcontract_generate::ContractBuilder;
use std::env;
use std::error::Error;
use std::path::Path;

const ARTIFACTS_PATH: &str = "infra/eth-provider/src/artifacts";
const CONTRACTS_DEST_PATH: &str = "infra/eth-provider/src/contracts";

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let network = &args[1];

    let root = project_root::get_project_root().expect("failed to get project");

    let artifacts_dir = Path::new(root.as_path()).join(ARTIFACTS_PATH);
    let contracts_dir = Path::new(root.as_path()).join(CONTRACTS_DEST_PATH);

    if !contracts_dir.is_dir() {
        std::fs::create_dir(&contracts_dir).unwrap_or_else(|_| panic!("failed to create_dir {:?}", &contracts_dir));
    }

    println!("loading artifacts ...");

    let artifacts = HardHatLoader::new()
        .allow_network_by_name(network)
        .load_from_directory(&artifacts_dir)
        .unwrap_or_else(|_| panic!("failed to load {:?}", &artifacts_dir));

    if artifacts.is_empty() {
        panic!(
            "{} has no artifacts. Please check '{:?}' directory and eth-provider/README.md",
            network, &artifacts_dir
        );
    }

    for contract in artifacts.iter() {
        let file_name = contract.name.to_case(Case::Snake);
        println!("generate {}.rs ...", file_name);

        let dest = contracts_dir.join(format!("{}.rs", file_name));

        let builder = ContractBuilder::new();
        builder
            .generate(contract)
            .expect("failed to generate")
            .write_to_file(dest)
            .expect("failed to write rust file");
    }

    Ok(())
}

// grcov: ignore-end
