use md5::{self, Digest};

fn main() {
    

    let mut suffix = 0;
    let input = String::from("ckczppom");

    loop
    {
        let hash_input = input.clone() + &suffix.to_string();

        let digest: Digest = md5::compute(&hash_input);

        let zeros_count = initial_zeros(digest);

        println!("I: {hash_input}, D:{:?}, C:{zeros_count}", digest);

        if zeros_count == 6
        {
            break;
        }

        suffix += 1;
    }

    println!("{suffix}");
}

fn initial_zeros(digest: Digest) -> usize
{
    let hash: String = format!("{:X}", digest);

    let (first_five, _rest) = hash.split_at(6);

    let number_of_zeroes: usize = first_five.chars().filter(|c| *c == '0').count();

    return number_of_zeroes;
}

#[test]
fn delivered_presents_test()
{
    let input: String = String::from("abcdef609043");
    let digest: Digest = md5::compute(&input);
    assert_eq!(initial_zeros(digest), 5);

    let input: String = String::from("pqrstuv1048970");
    let digest: Digest = md5::compute(&input);
    assert_eq!(initial_zeros(digest), 5);
}