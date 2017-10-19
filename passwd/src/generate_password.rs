use oath::{totp_raw_now, HashType};

pub fn generate_super_password(org_id: &str) -> u64 {
    totp_raw_now(org_id.as_bytes(), 6, 0, 3600, &HashType::SHA512)
}