#[derive(Debug)]
pub struct Line {
    pub time: f32,
    pub text: String,
}

pub fn parse_script(input: &str) -> Vec<Line> {
    input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(']').collect();
            if parts.len() != 2 {
                return None;
            }

            let time = parts[0]
                .trim_start_matches('[')
                .parse::<f32>()
                .ok()?;

            let text = parts[1].trim().to_string();

            Some(Line { time, text })
        })
        .collect()
}