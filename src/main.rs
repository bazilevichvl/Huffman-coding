use clap::Parser;
use huffman_coding::{encode, RawFile};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    input_file: String,
}

fn main() {
    let args = Args::parse();

    let input = RawFile::new(args.input_file);
    let raw_size = input.size();
    println!("Raw file size: {} bits", raw_size);
    let encoded = encode(&input);
    let encoded_size = encoded.size();
    println!("Encoded file size: {} bits", encoded_size);

    println!(
        "Compression ratio: {:.3}",
        raw_size as f64 / encoded_size as f64
    );
}
