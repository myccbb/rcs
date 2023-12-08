const PIECE_ID_LENGTH: u8 = 6;

pub fn random_piece_id() -> String {
    let now = crate::dist::utils::now_beijing();
    format!(
        "{}-{}-{}",
        now.format("%Y%m%d"),
        now.format("%H%M%S"),
        crate::dist::utils::random_alphabet_string(PIECE_ID_LENGTH)
    )
}
