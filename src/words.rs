use rand::seq::SliceRandom;
use rand::Rng;

pub const HACKER_WORDS: &[&str] = &[
    "firewall",
    "rootkit",
    "daemon",
    "kernel",
    "exploit",
    "cipher",
    "proxy",
    "backdoor",
    "payload",
    "shellcode",
    "overflow",
    "injection",
    "quantum",
    "neural",
    "matrix",
    "decrypt",
    "protocol",
    "entropy",
    "binary",
    "socket",
    "packet",
    "malware",
    "trojan",
    "botnet",
    "phishing",
    "ransomware",
    "spyware",
    "keylogger",
    "sandbox",
    "honeypot",
    "steganography",
    "cryptography",
    "algorithm",
    "blockchain",
    "bytecode",
    "compiler",
    "debugger",
    "firmware",
    "hash",
    "hypervisor",
    "interface",
    "jailbreak",
    "killswitch",
    "localhost",
    "metadata",
    "nonce",
    "obfuscate",
    "polymorphic",
    "quarantine",
    "recon",
    "sniffer",
    "terminal",
    "upstream",
    "vector",
    "worm",
    "xploit",
    "zerodday",
    "assembly",
    "bitshift",
    "cache",
    "datastream",
    "encryption",
    "fork",
    "gateway",
    "handshake",
    "interrupt",
    "jumphost",
    "kilobyte",
    "latency",
    "mutex",
    "namespace",
    "opcode",
    "pipeline",
    "queue",
    "register",
    "syscall",
    "thread",
    "unix",
    "volatile",
    "webhook",
    "xor",
    "yield",
    "zombie",
    "segfault",
    "deadlock",
    "spinlock",
    "semaphore",
    "coroutine",
    "epoll",
    "futex",
    "ioctl",
    "mmap",
    "ptrace",
    "sigkill",
    "strace",
    "valgrind",
    "netfilter",
    "iptables",
    "nftables",
    "wireguard",
    "tailscale",
];

pub const CODE_SNIPPETS: &[&str] = &[
    "fn main() {}",
    "let mut x = 0;",
    "impl Display for",
    "match result {",
    "async fn fetch()",
    "pub struct Node",
    "use std::io;",
    "println!(\"hello\");",
    "vec![1, 2, 3]",
    "Option<String>",
    "Result<(), Error>",
    "Box<dyn Trait>",
    "Arc<Mutex<T>>",
    "#[derive(Debug)]",
    "tokio::spawn(async",
    "for i in 0..n {",
    "while let Some(x)",
    "if let Ok(val) =",
    "loop { break; }",
    ".unwrap_or_default()",
    ".iter().map(|x|",
    ".collect::<Vec<_>>()",
    "impl Iterator for",
    "type Error = Box<",
    "mod tests {",
    "#[cfg(test)]",
    "assert_eq!(a, b);",
    "pub fn new() -> Self",
    "self.inner.lock()",
    "&'static str",
    "def __init__(self):",
    "import numpy as np",
    "from torch import nn",
    "class Model(nn.Module):",
    "self.forward(x)",
    "with open(f) as fp:",
    "yield from gen()",
    "async def main():",
    "raise ValueError()",
    "lambda x: x * 2",
    "list(map(int, s))",
    "@staticmethod",
    "try: except: pass",
    "os.path.join(a, b)",
    "json.loads(data)",
    "subprocess.run(cmd)",
];

pub const SYSTEM_COMMANDS: &[&str] = &[
    "sudo systemctl restart",
    "cargo build --release",
    "git rebase -i HEAD~3",
    "docker compose up -d",
    "ssh user@remote",
    "rsync -avz src/ dst/",
    "find . -name '*.rs'",
    "grep -rn TODO src/",
    "tar xzf archive.tar.gz",
    "chmod 755 script.sh",
    "curl -sL url | bash",
    "pacman -Syu --noconfirm",
    "journalctl -fu service",
    "ip addr show",
    "ss -tlnp",
    "lsblk -f",
    "mount /dev/sda1 /mnt",
    "dd if=/dev/zero of=disk",
    "strace -p 1234",
    "perf record -g ./bin",
    "objdump -d binary",
    "nm -C libfoo.so",
    "readelf -h binary",
    "gdb -batch -ex run",
    "valgrind --leak-check",
    "awk '{print $1}' log",
    "sed -i 's/old/new/g'",
    "xargs -I{} cp {} dst/",
    "watch -n1 nvidia-smi",
    "htop --sort-key PERCENT",
    "tmux new -s work",
    "screen -r session",
    "nmap -sV 10.0.0.0/24",
    "tcpdump -i eth0 port 80",
    "openssl s_client -connect",
    "certbot renew --dry-run",
    "ansible-playbook site.yml",
    "terraform plan -out=tf",
    "kubectl get pods -A",
    "helm upgrade --install",
];

/// Difficulty tiers for word length
const EASY_MAX_LEN: usize = 7;
const MEDIUM_MAX_LEN: usize = 10;

pub fn get_words_for_round(mode: &str, difficulty: u32) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let source: &[&str] = match mode {
        "hacker" => HACKER_WORDS,
        "code" => CODE_SNIPPETS,
        "commands" => SYSTEM_COMMANDS,
        _ => HACKER_WORDS,
    };

    let mut pool: Vec<&str> = source.to_vec();

    // Filter by difficulty for hacker words (single words have meaningful length)
    if mode == "hacker" {
        pool = match difficulty {
            0..=2 => pool
                .into_iter()
                .filter(|w| w.len() <= EASY_MAX_LEN)
                .collect(),
            3..=5 => pool
                .into_iter()
                .filter(|w| w.len() <= MEDIUM_MAX_LEN)
                .collect(),
            _ => pool,
        };
    }

    pool.shuffle(&mut rng);
    let count = rng.gen_range(30..=60);
    pool.into_iter()
        .cycle()
        .take(count)
        .map(|s| s.to_string())
        .collect()
}

pub fn get_cascade_word(difficulty: u32) -> String {
    let mut rng = rand::thread_rng();
    let pool: Vec<&&str> = match difficulty {
        0..=2 => HACKER_WORDS
            .iter()
            .filter(|w| w.len() <= EASY_MAX_LEN)
            .collect(),
        3..=5 => HACKER_WORDS
            .iter()
            .filter(|w| w.len() <= MEDIUM_MAX_LEN)
            .collect(),
        _ => HACKER_WORDS.iter().collect(),
    };
    pool.choose(&mut rng).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_word_count_in_expected_range() {
        for _ in 0..50 {
            let words = get_words_for_round("hacker", 0);
            assert!(
                (30..=60).contains(&words.len()),
                "got {} words",
                words.len()
            );
            assert!(words.iter().all(|w| !w.is_empty()));
        }
    }

    #[test]
    fn easy_difficulty_filters_hacker_words_by_length() {
        // Difficulty 0..=2 caps hacker words at EASY_MAX_LEN.
        for _ in 0..50 {
            for w in get_words_for_round("hacker", 0) {
                assert!(w.len() <= EASY_MAX_LEN, "{:?} exceeds easy length cap", w);
            }
        }
    }

    #[test]
    fn medium_difficulty_widens_length_cap() {
        for _ in 0..50 {
            for w in get_words_for_round("hacker", 4) {
                assert!(w.len() <= MEDIUM_MAX_LEN, "{:?} exceeds medium cap", w);
            }
        }
    }

    #[test]
    fn code_mode_draws_only_from_code_snippets() {
        for _ in 0..20 {
            for w in get_words_for_round("code", 0) {
                assert!(
                    CODE_SNIPPETS.contains(&w.as_str()),
                    "{:?} is not a code snippet",
                    w
                );
            }
        }
    }

    #[test]
    fn unknown_mode_falls_back_to_hacker_words() {
        for w in get_words_for_round("totally-unknown", 9) {
            assert!(HACKER_WORDS.contains(&w.as_str()));
        }
    }

    #[test]
    fn cascade_word_respects_easy_length_cap() {
        for _ in 0..100 {
            let w = get_cascade_word(0);
            assert!(w.len() <= EASY_MAX_LEN, "{:?} too long for easy", w);
            assert!(HACKER_WORDS.contains(&w.as_str()));
        }
    }
}
