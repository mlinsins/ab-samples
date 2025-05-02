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

use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub commit_hash: String,
    pub artifact_hash: String,
    pub artifact_name: String,
    pub attestation_document: String,
}

impl LogEntry {
    pub fn to_byte_array(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        result.extend(self.commit_hash.as_bytes());
        result.extend(self.artifact_hash.as_bytes());
        result.extend(self.artifact_name.as_bytes());
        result.extend(self.attestation_document.as_bytes());
        result
    }

    pub fn to_merkle_hash(&self) -> String {
        let mut result: Vec<u8> = vec![];
        result.push(0);
        result.extend(self.to_byte_array());
        let digest = Sha256::new().chain_update(result).finalize()[..].to_vec();
        general_purpose::STANDARD.encode(digest)
    }
}
