use std::string::ToString;

// 実際にはもう少し複雑な名前
fn stringify_i32(t: i32) -> String {
    <i32 as ToString>::to_string(t)
}
