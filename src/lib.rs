pub fn build_meaning_curve(code: &str) -> String {
    // 아주 간단한 더미 엔진
    // 실제 엔진은 너랑 같이 점점 발전시키면 됨
    let lines: Vec<&str> = code.lines().collect();
    let mut result = String::new();

    for (i, line) in lines.iter().enumerate() {
        let weight = (line.len() as f64) / 100.0;
        result.push_str(&format!("{}: {:.3}\n", i, weight));
    }

    result
}
