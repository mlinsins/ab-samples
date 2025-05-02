/*
** Copyright (C) 2024  Johannes Kepler University Linz, Institute of Networks and Security
** Copyright (C) 2024  CDL Digidow <https://www.digidow.eu/>
**
** Licensed under the EUPL, Version 1.2 or – as soon they will be approved by
** the European Commission - subsequent versions of the EUPL (the "Licence").
** You may not use this work except in compliance with the Licence.
**
** You should have received a copy of the European Union Public License along
** with this program.  If not, you may obtain a copy of the Licence at:
** <https://joinup.ec.europa.eu/software/page/eupl>
**
** Unless required by applicable law or agreed to in writing, software
** distributed under the Licence is distributed on an "AS IS" basis,
** WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
** See the Licence for the specific language governing permissions and
** limitations under the Licence.
**
*/

use clap::Parser;

use crate::models::log_entry::LogEntry;
use dotenv::dotenv;

mod models;
mod transparency_service;
mod verification_service;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(long, default_value = "http://localhost:8090", env = "VERIFIER_PERSONALITY_BASE_URL")]
    verifier_personality_base_url: String,

    #[clap(long, env = "VERIFIER_TREE_SIZE")]
    verifier_tree_size: i64,

    #[clap(long, env = "VERIFIER_LOG_ID")]
    verifier_log_id: String,

    #[clap(long)]
    commit_hash: String,

    #[clap(long)]
    artifact_hash: String,

    #[clap(long)]
    artifact_name: String,

    #[clap(long)]
    attestation_document: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;

    let args = Args::parse();

    let log_entry = LogEntry {
        commit_hash: args.commit_hash.to_string(),
        artifact_hash: args.artifact_hash.to_string(),
        artifact_name: args.artifact_name.to_string(),
        attestation_document: args.attestation_document.to_string(),
    };

    let result = transparency_service::request_inclusion_proof(
        args.verifier_personality_base_url,
        args.verifier_log_id,
        args.verifier_tree_size,
        log_entry.clone(),
    )
    .await?;
    verification_service::validate_inclusion_proof(
        args.verifier_tree_size,
        log_entry.clone().to_merkle_hash(),
        result,
    )
    .await?;

    Ok(())
}
