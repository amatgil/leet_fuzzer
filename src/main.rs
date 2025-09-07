use itertools::Itertools;
use rayon::prelude::*;
use std::time::{Duration, SystemTime};
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
        ([1, 0, 4], Value::Num(Array::from(4.0))),
        ([1, 0, 3], Value::Num(Array::from(3.0))),
        ([1, 0, 2], Value::Num(Array::from(2.0))),
        ([1, 0, 1], Value::Num(Array::from(1.0))),
    ];
    let checker = "for(len|/+|matbydedup)";

    //let number_of_options: u64 = (prims.len() as u64).pow(len);

    //let mut per_thou_time = SystemTime::now();
    const HOW_OFTEN_TO_PRINT: usize = 5000;

    let permutations: Vec<Vec<_>> = prims.into_iter().permutations(len as usize).collect();
    let candidates = permutations
        .par_iter()
        .filter(|code| {
            //'outer: for permutation in opt.iter().permutations(opt.len()) {

            //let code = permutation.into_iter().collect::<String>();
            let code: String = code.iter().collect();

            //if i % HOW_OFTEN_TO_PRINT == 0 {
            //    eprintln!("Trying out {code}");
            //}

            //eprintln!("Running: {code}");
            for (expected_out, input) in &tests {
                let mut uiua =
                    Uiua::with_safe_sys().with_execution_limit(Duration::from_millis(50));
                uiua.push(input.clone());

                let Ok(_) = uiua.run_str(&code) else {
                    return false;
                };
                let Ok(_) = uiua.run_str(&checker) else {
                    return false;
                };
                let res = uiua.take_stack();
                if res != expected_out {
                    return false;
                }
                //return res == expected_out;
            }
            true
            //println!("FOUND CANDIDATE: {code}");
            //}
        })
        .collect::<Vec<_>>();

    println!("Candidates (of length '{len}') were:");
    for c in &candidates {
        let c: String = c.into_iter().collect();
        println!("\t'{c}'");
    }
}
