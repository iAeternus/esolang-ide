use encoding_rs::GBK;

/// 自动检测并转换编码（解决解释器输出显示乱码的问题）
pub fn decode_output(bytes: Vec<u8>) -> String {
    // 尝试 UTF-8
    if let Ok(s) = String::from_utf8(bytes.clone()) {
        return s;
    }

    // 再尝试 GBK
    let (cow, _, _) = GBK.decode(&bytes);
    cow.to_string()
}
