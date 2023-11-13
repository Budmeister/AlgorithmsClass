use std::{
    env,
    fs::File,
    io::{
        Write,
        BufReader,
        BufRead
    },
    num::ParseIntError,
};

mod order_statistic;
use order_statistic::{
    order_statistic_rp,
    order_statistic_mof,
};

mod error;
use error::OrdStatErr;

fn main() -> Result<(), OrdStatErr> {
    // read array from input file
    let args: Vec<_> = env::args().collect();
    let input_path;
    if let [_, input_path_] = &args[..] {
        input_path = input_path_;
    } else {
        println!("Usage: {} file.csv", args[0]);
        return OrdStatErr::Args.into();
    }
    if !input_path.ends_with(".csv") {
        println!("File must end in \".csv\", but was, \"{}\"", input_path);
        return OrdStatErr::Args.into();
    }
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);


    // create stat arrays
    // stats[n_ind, i]
    let mut rp_stats = Vec::new();
    let mut mof_stats = Vec::new();
    let mut ns = Vec::new();

    for line in buffered.lines() {
        let line = line?;
        
        // This may fail if there is a newline
        let arr: Vec<i32> = line
            .split(',')
            .map(|num| num.parse())
            .collect::<Result<Vec<i32>, ParseIntError>>()?;
        let n = arr.len();
        println!("Received array with length {}", n);

        ns.push(n);
        let mut rp_stat = Vec::new();
        let mut mof_stat = Vec::new();
    
        let mut count;
        for i in (0..n).step_by(10_000) {
            // sort using rp
            count = 0;
            order_statistic_rp(&arr, i, &mut count);
            rp_stat.push(count);

            // sort using mof
            count = 0;
            order_statistic_mof(&arr, i, &mut count);
            mof_stat.push(count);
        }
        rp_stats.push(rp_stat);
        mof_stats.push(mof_stat);
    
    }
    // save stats to two files
    // rp
    let rp_path = format!("{}_rp.csv", &input_path[0..input_path.len() - 4]);
    let mut rp_file = File::create(rp_path)?;
    /* 
     *        i -->
     *       0 1 2 3
     *  n  0
     *  |  1
     *  |  2
     *  V  3
     */
    for rp_stat in rp_stats {
        for count in rp_stat {
            write!(rp_file, "{},", count)?;
        }
        writeln!(rp_file)?;
    }

    // mof
    let mof_path = format!("{}_mof.csv", &input_path[0..input_path.len() - 4]);
    let mut mof_file = File::create(mof_path)?;
    /* 
     *        i -->
     *       0 1 2 3
     *  n  0
     *  |  1
     *  |  2
     *  V  3
     */
    for mof_stat in mof_stats {
        for count in mof_stat {
            write!(mof_file, "{},", count)?;
        }
        writeln!(mof_file)?;
    }

    // save ns to a file
    // let ns_path = format!("{}_ns.csv", &input_path[0..input_path.len() - 4]);
    // let mut ns_file = File::create(ns_path)?;
    // for n in ns {
    //     write!(ns_file, "{},", n)?;
    // }
    // writeln!(ns_file)?;
    
    Ok(())
}
