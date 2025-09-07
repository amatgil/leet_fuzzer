use itertools::Itertools;
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

    let len: usize = 4;
    let tests = [
        ([1, 0, 5], Value::Num(Array::from(5.0))),
        ([1, 0, 4], Value::Num(Array::from(4.0))),
        ([1, 0, 3], Value::Num(Array::from(3.0))),
        ([1, 0, 2], Value::Num(Array::from(2.0))),
        ([1, 0, 1], Value::Num(Array::from(1.0))),
    ];
    let checker = "for(len|/+|matbydedup)";

    let mut final_output = vec![];
    let number_of_options: u128 = choose(prims.len() as u128 + len as u128 - 1, len as u128)
        * (1..=len as u128).product::<u128>();

    let mut per_thou_time = SystemTime::now();
    const HOW_OFTEN_TO_PRINT: usize = 5000;

    let mut i = 0;
    for opt in prims.into_iter().combinations_with_replacement(len) {
        'outer: for permutation in opt.iter().permutations(opt.len()) {
            i += 1;
            let code = permutation.into_iter().collect::<String>();


            if i % HOW_OFTEN_TO_PRINT == 0 {
                let delta_time = per_thou_time.elapsed().unwrap();
                let delta_time = (number_of_options as u128 - i as u128) as f32
                    * delta_time.as_secs_f32() as f32
                    / HOW_OFTEN_TO_PRINT as f32;
                per_thou_time = SystemTime::now();
                eprintln!(
                    "Trying out ({i:06}/{number_of_options}): '{}' (ETA:{:.2}s ({:.1}m))",
                    code,
                    delta_time,
                    delta_time / 60.
                );
            }

            for (expected_out, input) in &tests {
                let mut uiua = Uiua::with_safe_sys().with_execution_limit(Duration::from_millis(50));
                uiua.push(input.clone());

                let Ok(_) = uiua.run_str(&code) else {
                    continue 'outer;
                };
                let Ok(_) = uiua.run_str(&checker) else {
                    continue 'outer;
                };
                let res = uiua.take_stack();
                if res != expected_out {
                    continue 'outer;
                }
            }
            println!("FOUND CANDIDATE: {code}");
            final_output.push(code.clone());
        }
    }

    println!("Candidates (of length '{len}') were:");
    for c in &final_output {
        println!("\t'{c}'");
    }
}
