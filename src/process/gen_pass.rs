use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

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
        password.push(*UPPER.choose(&mut rng).expect("UPPER is empty"));
        charsets.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("LOWER is empty"));
        charsets.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER is empty"));
        charsets.extend_from_slice(SYMOL);
        password.push(*SYMOL.choose(&mut rng).expect("SYMOL is empty"));
    }

    // println!("{:?}", charsets);

    for _ in 0..(length - password.len() as u8) {
        // let charset = charsets.choose(&mut rng).expect("charset is empty");
        // // let char = charset.chars().choose(&mut rng).unwrap();
        // password.push(*charset);
        password.push(*charsets.choose(&mut rng).expect("charsets is empty"));
    }
    println!("{:?}", password);
    password.shuffle(&mut rng);
    let password = String::from_utf8(password)?;
    println!("genpass is: {}", password);

    //output password strength in stderr
    let estimates = zxcvbn(&password, &[])?;
    eprintln!("password strength: {:?}", estimates.score());

    Ok(())
}
