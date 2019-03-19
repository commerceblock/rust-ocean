
extern crate bitcoin;
extern crate rust_ocean;

fn do_test(data: &[u8]) {
    let tx_result: Result<rust_ocean::Transaction, _> = bitcoin::consensus::deserialize(data);
    match tx_result {
        Err(_) => {},
        Ok(mut tx) => {
            let reser = bitcoin::consensus::serialize(&tx);
            assert_eq!(data, &reser[..]);
            let len = reser.len();
            let calculated_weight = tx.get_weight();
            for input in &mut tx.input {
                input.witness = rust_ocean::TxInWitness::default();
            }
            for output in &mut tx.output {
                output.witness = rust_ocean::TxOutWitness::default();
            }
            assert_eq!(tx.has_witness(), false);
            let no_witness_len = bitcoin::consensus::serialize(&tx).len();
            assert_eq!(no_witness_len * 3 + len, calculated_weight);

            for output in &tx.output {
                output.is_null_data();
                output.is_pegout();
                output.pegout_data();
                output.is_fee();
                output.minimum_value();
            }
        },
    }
}

#[cfg(feature = "afl")]
extern crate afl;
#[cfg(feature = "afl")]
fn main() {
    afl::read_stdio_bytes(|data| {
        do_test(&data);
    });
}

#[cfg(feature = "honggfuzz")]
#[macro_use] extern crate honggfuzz;
#[cfg(feature = "honggfuzz")]
fn main() {
    loop {
        fuzz!(|data| {
            do_test(data);
        });
    }
}

#[cfg(test)]
mod tests {
    fn extend_vec_from_hex(hex: &str, out: &mut Vec<u8>) {
        let mut b = 0;
        for (idx, c) in hex.as_bytes().iter().enumerate() {
            b <<= 4;
            match *c {
                b'A'...b'F' => b |= c - b'A' + 10,
                b'a'...b'f' => b |= c - b'a' + 10,
                b'0'...b'9' => b |= c - b'0',
                _ => panic!("Bad hex"),
            }
            if (idx & 1) == 1 {
                out.push(b);
                b = 0;
            }
        }
    }

    #[test]
    fn duplicate_crash() {
        let mut a = Vec::new();
        extend_vec_from_hex("00", &mut a);
        super::do_test(&a);
    }
}
