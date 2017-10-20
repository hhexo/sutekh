extern crate rand;
use rand::distributions::IndependentSample;

use std::env;
use std::error::Error;
use std::process;
use std::thread;
use std::time;


fn get_running_containers(kube: bool, filter: &str) -> Result<Vec<String>, String> {
    let child = if kube {
        match process::Command::new("kubectl")
        .arg("get")
        .arg("pods")
        .arg("-l")
        .arg(filter)
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn() {
            Ok(c) => c,
            Err(e) => { return Err(e.description().to_string()) },
        }
    } else {
        match process::Command::new("docker")
        .arg("ps")
        .arg("-f")
        .arg(format!("name={}", filter))
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn() {
            Ok(c) => c,
            Err(e) => { return Err(e.description().to_string()) },
        }
    };

    let output = match child.wait_with_output() {
        Ok(o) => o,
        Err(e) => { return Err(e.description().to_string()) },
    };

    if output.status.success() {
        let v = String::from_utf8_lossy(output.stdout.as_slice())
            .lines()
            .skip(1)
            .map(|l| match l.split_whitespace().next() {
                Some(s) => s.to_string(),
                None => "".to_string()
            })
            .filter(|s| !s.is_empty())
            .collect();
        Ok(v)
    } else {
        Err(format!("Can't get running containers: {}",
            String::from_utf8_lossy(output.stderr.as_slice())).into())
    }
}

fn kill_container(kube: bool, choice: String) -> Result<(), String> {
    println!("Killing {}...", &choice);
    let child = if kube {
        // TODO
        match process::Command::new("kubectl")
        .arg("delete")
        .arg("pod")
        .arg("--grace-period=10")
        .arg(&choice)
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn() {
            Ok(c) => c,
            Err(e) => { return Err(e.description().to_string()) },
        }
    } else {
        match process::Command::new("docker")
        .arg("rm")
        .arg("-f")
        .arg(&choice)
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn() {
            Ok(c) => c,
            Err(e) => { return Err(e.description().to_string()) },
        }
    };

    let output = match child.wait_with_output() {
        Ok(o) => o,
        Err(e) => { return Err(e.description().to_string()) },
    };

    if output.status.success() {
        println!("Killed {}.", &choice);
        Ok(())
    } else {
        Err(format!("Can't kill container: {}",
            String::from_utf8_lossy(output.stderr.as_slice())).into())
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut kube = false;
    let mut filter = String::new();
    env::args().skip(1).fold((), |_, a| {
        if a == "-k" || a == "--kube" {
            kube = true;
        }
        else {
            filter = a.to_string();
        }
    });
    if filter.is_empty() {
        println!("Please specify a partial name or label for which containers should be killed");
        println!("");
        println!("    sutekh [-k/--kube] PARTIAL_NAME_OR_LABEL");
        println!("");
        process::exit(2);
    }
    loop {
        // lambda = 0.1, mean = 10
        let exp = rand::distributions::exponential::Exp::new(0.1f64);
        let mut x = exp.ind_sample(&mut rng);
        if x < 0f64 {
            x = -x;
        }
        if x > 60f64 {
            x = 60f64;
        }
        println!("Sleeping {} seconds", x);
        thread::sleep(time::Duration::from_millis((x * 1000f64) as u64));
        println!("Getting '{}' containers...", filter);
        let containers = match get_running_containers(kube, &filter) {
            Ok(cs) => cs,
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        };
        if containers.len() > 0 {
            println!("Killing a container at random...");
            let mut choices = containers.iter().map(|c| rand::distributions::Weighted{
                weight: 1,
                item: c.clone()
            }).collect::<Vec<_>>();
            let wc = rand::distributions::WeightedChoice::new(&mut choices);
            let choice = wc.ind_sample(&mut rng);
            match kill_container(kube, choice) {
                Ok(_) => (),
                Err(e) => {
                    println!("{}", e);
                    process::exit(1);
                }
            }
        }
    }
}
