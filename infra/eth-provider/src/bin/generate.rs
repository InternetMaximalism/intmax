// grcov: ignore-start
use convert_case::{Case, Casing};

use ethcontract_generate::loaders::HardHatLoader;
use ethcontract_generate::ContractBuilder;
use std::error::Error;
use std::path::Path;
use std::{env};

const ARTIFACTS_PATH: &str = "infra/eth-provider/src/artifacts";
const CONTRACTS_DEST_PATH: &str = "infra/eth-provider/src/contracts";

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let network = &args[1];

    let mut dir = env::current_dir().expect("should get current_dir");
    println!("current_dir: {:?}", dir);

    // cd pj root
    loop {
        let is_pj_root = dir.ends_with("intmax");
        if is_pj_root {
            env::set_current_dir(&dir);
            break;
        }
        dir.pop();
    }

    println!("loading artifacts ...");

    let artifacts = HardHatLoader::new()
        .allow_network_by_name(network)
        .load_from_directory(ARTIFACTS_PATH)
        .expect(format!("failed to load {:?}", ARTIFACTS_PATH).as_str());

    if artifacts.len() == 0 {
        panic!(
            "{} has no artifacts. Please check '{}' directory and eth-provider/README.md",
            network, ARTIFACTS_PATH
        );
    }

    for contract in artifacts.iter() {
        let c = contract.clone();

        let file_name = c.name.clone().to_case(Case::Snake);
        println!("generate {}.rs ...", file_name);

        let dest =
            Path::new(&dir).join(format!("infra/eth-provider/src/contracts/{}.rs", file_name));

        let builder = ContractBuilder::new();
        builder
            .generate(&contract)
            .expect("failed to generate")
            .write_to_file(dest)
            .expect("failed to write rust file");
    }

    Ok(())
}

// grcov: ignore-end
