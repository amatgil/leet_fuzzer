use itertools::Itertools;
use rayon::prelude::*;
use rayon_progress::ProgressAdaptor;
use std::panic;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};
use uiua::*;

fn choose(n: u128, mut k: u128) -> u128 {
    if k > n {
        return 0;
    }
    if k * 2 > n {
        k = n - k
    }
    if k == 0 {
        return 1;
    }

    let mut result = n;
    for i in 2..=k {
        result *= n - i + 1;
        result /= i;
    }
    result
}

fn main() {
    // Note that space is also a primitive!! >:D
    let prims: Vec<char> = r".:◌⟜⊸⤙⤚◡˙˜ητ∞¬±¯⌵√ₑ∿⌊⌈⁅=≠<≤>≥+-×÷◿ⁿ↧↥∠ℂ⧻△⇡⊢⊣⇌♭¤⋯⍉⍆⍏⍖⊚◴⊛⧆□≍⊟⊂⊏⊡↯↙↘↻⤸▽⌕⦷∊⨂≡⍚⊞⧅⧈⍥⍢/∧\⊕⊜⌅°⌝⍜∘⋅⊙∩⊃⊓◇⬚⨬"
        // don't forget to add base after the bug is fixed
     .chars()
     .collect();
    //let prims: Vec<char> = r" .:◌⟜⊸⤙⤚◠◡˙˜ητ∞-⇌⊸⇡".chars().collect();

    let len: u32 = 4;
    let tests = [
        ([1, 0, 5], Value::Num(Array::from(5.0))),
        //([1, 0, 4], Value::Num(Array::from(4.0))),
        ([1, 0, 3], Value::Num(Array::from(3.0))),
        //([1, 0, 2], Value::Num(Array::from(2.0))),
        //([1, 0, 1], Value::Num(Array::from(1.0))),
    ];
    let checker = "for(len|/+|matbydedup)";

    let number_of_options: u64 = (prims.len() as u64).pow(len);

    let permutations: Vec<Vec<_>> = prims.into_iter().permutations(len as usize).collect();

    let iterator = ProgressAdaptor::new(permutations);
    let progress = iterator.items_processed();
    let result = Arc::new(Mutex::new(None));

    rayon::spawn({
        let result = result.clone();
        move || {
            let candidates = iterator
                .filter(|code| {
                    let code: String = code.iter().collect();
                    eprintln!("START: '{code}");
                    for (expected_out, input) in &tests {
                        let mut uiua =
                            Uiua::with_safe_sys().with_execution_limit(Duration::from_millis(50));
                        uiua.push(input.clone());
                        let Ok(_) = uiua.run_str(&code) else {
                            eprintln!("END: '{code}");
                            return false;
                        };
                        let Ok(_) = uiua.run_str(&checker) else {
                            eprintln!("END: '{code}");
                            return false;
                        };
                        let res = uiua.take_stack();
                        if res != expected_out {
                            eprintln!("END: '{code}");
                            return false;
                        }
                    }
                    eprintln!("END: '{code}");
                    true
                })
                .collect::<Vec<_>>();

            *result.lock().unwrap() = Some(candidates);
        }
    });

    let mut last_print = SystemTime::now();
    while result.lock().unwrap().is_none() {
        if last_print.elapsed().unwrap().as_millis() > 1000 {
            let percent = (progress.get() * 100) as f32 / number_of_options as f32;
            println!("Processing... {:.2}% complete", percent);
            last_print = SystemTime::now();
        }
    }

    println!("Candidates (of length '{len}') were:");
    for c in &*result.lock().unwrap().as_ref().unwrap() {
        let c: String = c.clone().into_iter().collect();
        println!("\t'{c}'");
    }
}
