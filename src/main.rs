use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[derive(Debug, PartialEq)]
struct Claim<'a> {
    pub dimension: &'a str,
    pub x: i64,
    pub z: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    // let input = "{\n\tmax_claim_chunks: 500\n\tmax_force_load_chunks: 25\n\tlast_login_time: 1727649883449L\n\tchunks: { }\n\tmember_data: {\n\t\t17eef99f-8754-4408-bd4d-aad74714be4c: {\n\t\t\tmax_claimed_chunks: 500\n\t\t\toffline_force_loader: 0b\n\t\t\tmax_force_loaded_chunks: 25\n\t\t}\n\t}\n}";
    // let input = "{\n\tchunks: {\n\t\t\"minecraft:overworld\": [\n\t\t\t{ x: -28, z: -18, time: 1724521821807L }\n\t\t]\n\t}\n}";

    #[test]
    fn test_parse_one() {
        let input = "{ x: 14, z: 0, time: 1724521821807L }";
        let actual = parse_claim(input).unwrap();
        let expected = Claim {
            dimension: "minecraft:overworld",
            x: 14,
            z: 0,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_another() {
        let input = "{ x: -28, z: -18, time: 1724521821807L }";
        let actual = parse_claim(input).unwrap();
        let expected = Claim {
            dimension: "minecraft:overworld",
            x: -28,
            z: -18,
        };
        assert_eq!(actual, expected);
    }
}

fn parse_claim(input: &str) -> Result<Claim, Box<dyn Error>> {
    if &input[0..5] != "{ x: " {
        return Err(Box::<dyn Error>::from("expected exactly { x: "));
    }

    let input = &input[5..];
    let index = input.find(|c: char| c != '-' && !c.is_numeric()).unwrap();
    let x_str = &input[0..index];
    let input = &input[index..];

    if &input[0..5] != ", z: " {
        return Err(Box::<dyn Error>::from("expected exactly , z: "));
    }

    let input = &input[5..];
    let index = input.find(|c: char| c != '-' && !c.is_numeric()).unwrap();
    let z_str = &input[0..index];

    Ok(Claim {
        dimension: "minecraft:overworld",
        x: x_str.parse().unwrap(),
        z: z_str.parse().unwrap(),
    })
}
