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
use base64::{engine::general_purpose, Engine as _};

use sha2::{Digest, Sha256};

pub async fn validate_inclusion_proof(
    tree_size: i64,
    merkle_hash: String,
    inclusion_proof: InclusionProof,
) -> anyhow::Result<()> {
    let local_root_node = calculate_root_node(
        inclusion_proof.leaf_index,
        merkle_hash,
        tree_size,
        inclusion_proof.hashes,
    );
    println!("Local root node: {}", local_root_node);
    let root_node_hash = get_root_node_from_signed_log_root(inclusion_proof.log_root.clone());
    println!("Signed root node: {}", root_node_hash);
    Ok(())
}

fn calculate_root_node(
    leaf_index: i64,
    merkle_hash: String,
    tree_size: i64,
    ordered_hashes: Vec<String>,
) -> String {
    let current_index = leaf_index;
    let inner = i64::BITS - (current_index ^ (tree_size - 1)).leading_zeros();

    let mut result = merkle_hash.clone();
    let mut left: String;
    let mut right: String;

    for i in 0..ordered_hashes.len() {
        if i < inner as usize && (((leaf_index >> i) & 1) == 0) {
            left = merkle_hash.clone();
            right = ordered_hashes.get(i).unwrap().clone();
        } else {
            left = ordered_hashes.get(i).unwrap().clone();
            right = result.clone();
        }
        result = calculate_inner_node(left, right);
    }
    result
}

fn calculate_inner_node(left: String, right: String) -> String {
    let mut result: Vec<u8> = vec![];
    result.push(1);
    result.extend(general_purpose::STANDARD.decode(left).unwrap());
    result.extend(general_purpose::STANDARD.decode(right).unwrap());
    let digest = Sha256::new().chain_update(result).finalize()[..].to_vec();
    general_purpose::STANDARD.encode(digest)
}

fn get_root_node_from_signed_log_root(signed_log_root: String) -> String {
    let root_node_bytes = general_purpose::STANDARD.decode(signed_log_root).unwrap();
    let length = *root_node_bytes.get(10).unwrap();
    let root_hash = root_node_bytes.get(11..11 + length as usize).unwrap();
    general_purpose::STANDARD.encode(root_hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::inclusion_proof::InclusionProof;
    use crate::models::log_entry::LogEntry;

    #[tokio::test]
    async fn test_validate_inclusion_proof() {
        let log_entry = LogEntry {
            commit_hash: "commit-hash-test".to_string(),
            artifact_hash: "artifact-hash-test".to_string(),
            artifact_name: "artifact-name-test".to_string(),
            attestation_document: "attestation-test-document".to_string(),
        };

        let inclusion_proof = InclusionProof {
        leaf_index: 2,
        hashes: vec!["6ZUlW4u64TBQdglXv3GMVwWGGCOTy2zxisNjxp5OjjE=".to_string()],
        log_root: "AAEAAAAAAAAAAyBScXQ9WT6+lrO85fRp0bppOOxIS+yLzlWSaS5W8s/LwhgKC7G7nUHCAAAAAAAAAAAAAA==".to_string()
      };

        let local_root_node = calculate_root_node(
            inclusion_proof.leaf_index,
            log_entry.to_merkle_hash(),
            3,
            inclusion_proof.hashes,
        );
        println!("Local root node: {}", local_root_node);
        assert!(local_root_node == "fYkovMfBI+eo1CuYXfCuioKP6BO5zf97MdKujS9HfsM=");

        let root_node_hash = get_root_node_from_signed_log_root(inclusion_proof.log_root.clone());
        assert!(root_node_hash == "UnF0PVk+vpazvOX0adG6aTjsSEvsi85VkmkuVvLPy8I=");
    }
}
