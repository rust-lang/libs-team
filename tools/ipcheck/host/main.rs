use serde_json::{Map, Value};
use std::{collections::BTreeSet, fmt::Write, io::BufRead, net::IpAddr};

const REF_LANG: &'static str = "Rust (New)";

fn is_ref(lang: &str) -> bool {
    lang.starts_with("Rust")
}

fn is_unsupported(value: impl PartialEq<&'static str>) -> bool {
    value == "<unsupported>"
}

fn main() {
    let (ref_artifact, impls) = &parse_impls();
    let addrs = parse_addrs();

    // Write a table header for the input and tested languages
    let mut header_buffer = String::new();
    let mut separator_buffer = String::new();

    write!(header_buffer, "| addr |").unwrap();
    write!(separator_buffer, "| - |").unwrap();

    for (lang, _) in impls {
        write!(header_buffer, " {} |", lang).unwrap();
        write!(separator_buffer, " - |").unwrap();
    }

    println!("{}", header_buffer);
    println!("{}", separator_buffer);

    // Operations that have no support in non-reference implementations
    let mut no_lang_support = BTreeSet::new();

    for addr in &addrs {
        print!("| `{}` |", addr);

        let ref_output = invoke_impl(REF_LANG, ref_artifact, addr);
        no_lang_support.extend(ref_output.keys().cloned());

        for (lang, artifact) in impls {
            let output = invoke_impl(lang, artifact, addr);

            // Make sure the output has the same shape as our reference reference
            assert_eq!(
                ref_output.keys().collect::<Vec<_>>(),
                output.keys().collect::<Vec<_>>(),
                "Rust methods don't match {} methods",
                lang
            );

            let mut buffer = String::new();
            let mut diffs = 0;

            for ((ref_key, ref_value), (_, value)) in ref_output.iter().zip(output.iter()) {
                if !is_ref(lang) && !is_unsupported(value) {
                    no_lang_support.remove(ref_key);
                }

                let values_match = {
                    let parsed_ref_value: Option<IpAddr> =
                        ref_value.as_str().and_then(|addr| addr.parse().ok());
                    let parsed_value: Option<IpAddr> =
                        value.as_str().and_then(|addr| addr.parse().ok());

                    match (parsed_ref_value, parsed_value) {
                        // If we can parse the addresses then check them
                        // This avoids some false negatives on formatting differences
                        (Some(ref_value), Some(parsed_value)) => ref_value == parsed_value,
                        _ => ref_value == value,
                    }
                };

                // If the values don't match, and they're not simply unsupported
                // then append the offending operation to the diff
                if !values_match && !is_unsupported(value) && !is_unsupported(ref_value) {
                    if diffs > 0 {
                        write!(buffer, ", ").unwrap();
                    }

                    write!(
                        buffer,
                        "{} : {} ({}) ≠ {} ({})",
                        ref_key, ref_value, REF_LANG, value, lang
                    )
                    .unwrap();

                    diffs += 1;
                }
            }

            if diffs == 0 {
                print!(" ✔️ |")
            } else {
                print!(" ❌ `{{ {} }}` |", buffer);
            }
        }

        println!();
    }

    // Write a list of operations with no support from other languages
    if no_lang_support.len() > 0 {
        println!();
        println!("Operations with only a Rust implementation:");

        for unsupported in no_lang_support {
            println!("- `{}`", unsupported);
        }
    }
}

fn parse_impls() -> (String, Vec<(String, String)>) {
    let mut impls: Vec<(String, String)> = std::fs::read("../artifacts/.impls")
        .expect("missing impls file")
        .lines()
        .map(|line| line.expect("invalid impls file"))
        .map(|line| {
            let mut line = line.split(": ");
            let lang = line.next().expect("missing impl language id");
            let artifact = line.next().expect("missing impl artifact path");

            println!("Found {} implementation.", lang);

            (lang.into(), artifact.into())
        })
        .collect();

    if impls.is_empty() || impls.get(0).map_or(false, |(lang, _)| lang != REF_LANG) {
        println!("No reference implementation found.");
        std::process::exit(0);
    }

    let ref_lang = impls.remove(0);

    if impls.is_empty() {
        println!("No implementations to compare against found.");
        std::process::exit(0);
    }

    println!();

    (ref_lang.1, impls)
}

fn parse_addrs() -> Vec<String> {
    let addrs: Vec<String> = std::fs::read("input.txt")
        .expect("missing input file")
        .lines()
        .filter_map(|line| {
            let line = line.expect("invalid input file");

            if line.starts_with("#") || line.is_empty() {
                None
            } else {
                Some(line.into())
            }
        })
        .collect();

    if addrs.is_empty() {
        println!("No addresses found.");
        std::process::exit(0);
    }

    addrs
}

fn invoke_impl(lang: &str, artifact: &str, addr: &str) -> Map<String, Value> {
    let mut artifact = artifact.split(" ");
    let mut command = std::process::Command::new(
        artifact
            .next()
            .expect(&format!("missing {} artifact", lang)),
    );
    for arg in artifact {
        command.arg(arg);
    }

    let output = command
        .arg(addr)
        .output()
        .expect(&format!("failed to invoke {} artifact", lang));

    if output.stderr.is_empty() {
        let out = String::from_utf8(output.stdout)
            .expect(&format!("failed to parse {} artifact output", lang));
        serde_json::from_str(&out).expect(&format!(
            "failed to parse {} artifact output {:?}",
            lang, out
        ))
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        panic!("{} impl error: {:?}", lang, err)
    }
}
