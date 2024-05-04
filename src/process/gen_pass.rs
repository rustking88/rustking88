use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMOL: &[u8] = b"!@#$%^&*()_";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symol: bool,
) -> anyhow::Result<()> {
    let mut password = Vec::new();
    let mut rng = rand::thread_rng();
    let mut charsets = Vec::new();

    if upper {
        charsets.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("UPPER is empty"));
    }
    if lower {
        charsets.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("LOWER is empty"));
    }
    if number {
        charsets.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER is empty"));
    }
    if symol {
        charsets.extend_from_slice(SYMOL);
        password.push(*SYMOL.choose(&mut rng).expect("SYMOL is empty"));
    }

    if charsets.is_empty() {
        charsets.extend_from_slice(UPPER);
        charsets.extend_from_slice(LOWER);
        charsets.extend_from_slice(NUMBER);
        charsets.extend_from_slice(SYMOL);
    }

    println!("{:?}", charsets);

    for _ in 0..(length - password.len() as u8) {
        // let charset = charsets.choose(&mut rng).expect("charset is empty");
        // // let char = charset.chars().choose(&mut rng).unwrap();
        // password.push(*charset);
        password.push(*charsets.choose(&mut rng).expect("charsets is empty"));
    }
    println!("{:?}", password);
    password.shuffle(&mut rng);
    println!("{:?}", password);
    println!("{}", String::from_utf8(password)?);
    // println!("{:?}", password);
    Ok(())
}
