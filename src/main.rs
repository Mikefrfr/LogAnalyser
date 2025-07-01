use std::io::{self, Write};
use std::process::Command;
use std::collections::HashMap;
use regex::Regex;
use colored::*;


fn main() {
    loop {
        println!("Detect brute-force attempts for:");
        println!("1. SSH");
        println!("2. FTP (vsftpd)");
        println!("3. OpenVPN");
        println!("4. SMTP");

        print!("Enter option (1/2/3/4): ");
        io::stdout().flush().unwrap();
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read input");

        print!("Enter start time (e.g., 'today', '2024-06-24 14:00:00'): ");
        io::stdout().flush().unwrap();
        let mut since = String::new();
        io::stdin().read_line(&mut since).expect("Failed to read input");

        print!("Enter end time (e.g., 'now', '2024-06-24 16:00:00'): ");
        io::stdout().flush().unwrap();
        let mut until = String::new();
        io::stdin().read_line(&mut until).expect("Failed to read input");

        print!("Enter threshold (number of failed attempts): ");
        io::stdout().flush().unwrap();
        let mut threshold = String::new();
        io::stdin().read_line(&mut threshold).expect("Failed to read input");
        let threshold: usize = threshold.trim().parse().expect("Enter a valid number");

        let option = option.trim();
        let since = since.trim();
        let until = until.trim();

        let comm: &str;
        let regex: Regex;
        let group_index = 1;

        if option == "1" {
            println!("1. Password logins");
            println!("2. Public key logins");
            print!("Enter option (1/2): ");
            io::stdout().flush().unwrap();
            let mut sshopt = String::new();
            io::stdin().read_line(&mut sshopt).expect("Failed to read input");

            comm = "sshd";
            if sshopt.trim() == "1" {
                regex = Regex::new(r"Failed password for \S+ from (\d+\.\d+\.\d+\.\d+)").unwrap();
            } else if sshopt.trim() == "2" {
                regex = Regex::new(r"Failed publickey for \S+ from (\d+\.\d+\.\d+\.\d+)").unwrap();
            } else {
                println!("Invalid SSH option");
                continue;
            }
        } else if option == "2" {
            comm = "vsftpd";
            regex = Regex::new(r#"FAIL LOGIN: Client "::ffff:(\d+\.\d+\.\d+\.\d+)""#).unwrap();
        } else if option == "3" {
            println!("1. TLS Errors");
            println!("2. AUTH_FAILED Errors");
            print!("Enter option (1/2): ");
            io::stdout().flush().unwrap();
            let mut vpnopt = String::new();
            io::stdin().read_line(&mut vpnopt).expect("Failed to read input");

            comm = "openvpn";
            if vpnopt.trim() == "1" {
                regex = Regex::new(r"TLS Error:.*from \[AF_INET\](\d+\.\d+\.\d+\.\d+)").unwrap();
            } else if vpnopt.trim() == "2" {
                regex = Regex::new(r"AUTH_FAILED.*?,(\w+)").unwrap();
            } else {
                println!("Invalid OpenVPN option");
                continue;
            }
        } else if option == "4" {
            println!("1. Postfix");
            println!("2. Dovecot");
            print!("Enter option (1/2): ");
            io::stdout().flush().unwrap();
            let mut smtpopt = String::new();
            io::stdin().read_line(&mut smtpopt).expect("Failed to read input");

            if smtpopt.trim() == "1" {
                comm = "postfix/smtpd";
                regex = Regex::new(r"SASL \w+ authentication failed.*\[(\d+\.\d+\.\d+\.\d+)\]").unwrap();
            } else if smtpopt.trim() == "2" {
                comm = "dovecot";
                regex = Regex::new(r"Failed login.*rip=(\d+\.\d+\.\d+\.\d+)").unwrap();
            } else {
                println!("Invalid SMTP option");
                continue;
            }
        } else {
            println!("Invalid option");
            continue;
        }

        let output = Command::new("journalctl")
            .args(&["-u", comm, "--no-pager", "--since", since, "--until", until])
            .output()
            .expect("Failed to run journalctl");

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error: {}", err);
            continue;
        }

        let logs = String::from_utf8_lossy(&output.stdout);
        if logs.contains("-- No entries --") || logs.trim().is_empty() {
            println!("{}", "Logs are empty!".yellow());
            continue;
        }

        let mut ip_counts: HashMap<String, usize> = HashMap::new();
        for line in logs.lines() {
            if let Some(cap) = regex.captures(line) {
                if let Some(ip) = cap.get(group_index) {
                    let ip_str = ip.as_str().to_string();
                    *ip_counts.entry(ip_str).or_insert(0) += 1;
                }
            }
        }

        let mut alert = false;
        for (ip, count) in &ip_counts {
            if *count >= threshold {
                println!("{}", format!("{} has {} failed login attempts!", ip, count).red().bold());
                alert = true;
            }
        }

        if !alert {
            println!("{}", "No failed login attempts above threshold.".green());
        }

        // Ask to rerun or exit
        println!("\nDo you want to analyze again? (y/n): ");
        io::stdout().flush().unwrap();
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("Failed to read input");
        if again.trim().eq_ignore_ascii_case("n") || again.trim().eq_ignore_ascii_case("q") || again.trim().eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }
    }
}
