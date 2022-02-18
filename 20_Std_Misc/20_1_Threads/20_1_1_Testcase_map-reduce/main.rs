use std::thread;

// This is the `main` thread
fn main() {
    // This is our data to process.
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    // Make a vector to hold the child-threads which we will spawn.
    let mut children = vec![];

    /*************************************************************************
     * "Map" phase
     *
     * Divide our data into segments, and apply initial processing
     ************************************************************************/
    // Join the original data into one big block.
    let full_data: Vec<char> = data.chars().filter(|x| *x != '\n').collect();

    // split the data into segments for individual calculation
    let num_threads: usize = 6;
    let chunk_size = (full_data.len() as f32 / num_threads as f32).ceil() as usize;
    let chunked_data = full_data.chunks(chunk_size);
    println!(
        "Full data length is {}. Thread count is {}. Block size is {}.",
        full_data.len(),
        num_threads,
        chunk_size
    );

    // Enumerate the chunked data and spawn threads.
    for (i, data_segment) in chunked_data.enumerate() {
        let cloned_segment = data_segment.to_owned();
        print!("Data segment {} is ", i);
        for c in data_segment {
            print!("{}", c);
        }
        println!("");

        children.push(thread::spawn(move || -> u32 {
            // Calculate the intermediate sum of this segment:
            let result = cloned_segment
                .iter()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();
            println!("processed segment {}, result={}", i, result);
            result
        }))
    }

    /*************************************************************************
     * "Reduce" phase
     *
     * Collect our intermediate results, and combine them into a final result
     ************************************************************************/
    let final_result: u32 = children.into_iter().map(|c| c.join().unwrap()).sum();

    println!("Final sum result: {}", final_result);
}
