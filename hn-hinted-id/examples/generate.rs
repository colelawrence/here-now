use hn_hinted_id::HintedID;

fn main() {
    // get first argument from command
    let mut args = std::env::args().skip(1);
    while let (Some(prefix), count_opt) = (args.next(), args.next()) {
        let count = count_opt
            .map(|st| -> usize {
                // try_from(st).expect("next argument is a number")
                st.parse().expect("next argument is a number")
            })
            .unwrap_or(1);
        for _ in 0..count {
            std::thread::sleep(std::time::Duration::from_millis(500));
            let prefix_clone = prefix.clone();
            std::thread::spawn(move || {
                println!("{}", HintedID::generate(&prefix_clone));
            })
            .join()
            .unwrap();
        }
    }
}
