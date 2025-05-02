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

use crate::models::inclusion_proof::InclusionProof;
use crate::models::log_entry::LogEntry;
use reqwest::Client;

pub async fn request_inclusion_proof(
    transparency_log_base_url: String,
    log_id: String,
    tree_size: i64,
    log_entry: LogEntry,
) -> anyhow::Result<InclusionProof> {
    let endpoint = format!(
        "/log/inclusion-proof?log_id={}&tree_size={}",
        log_id, tree_size
    );
    let url = format!("{}{}", transparency_log_base_url, endpoint);

    println!("Requesting inclusion proof from {}", url);

    let client = Client::new();
    let response = client.post(&url).json(&log_entry).send().await?;
    if response.status().is_success() {
        let body = response.text().await?;
        println!("Inclusion proof received: {}", body);
        let inclusion_proofs: Vec<InclusionProof> = serde_json::from_str(&body)?;
        if let Some(inclusion_proof) = inclusion_proofs.first() {
            Ok(inclusion_proof.clone())
        } else {
            Err(anyhow::anyhow!("No inclusion proof found"))
        }
    } else {
        println!("Failed to get inclusion proof: {}", response.status());
        Err(anyhow::anyhow!("Failed to get inclusion proof"))
    }
}
