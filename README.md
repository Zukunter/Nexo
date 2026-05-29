# Nexus

### Nexus is a lightweight tool designed to **nest files in any way you want**. 

---

## Wiki
- **[Nexus Cennter File](ca://s?q=)**: Define a nexus file where are all rules so dont have to rewrite yourself.
- **[Commands](ca://s?q=Nexus_CLI_interface)**: Simple commands to nest files or directories and execute commands.

--- 

## Caracteristics
- **Size** : 500 KB
- **Language** : Completely written in Rust 
- **Multithread** : Yes 

---

##  Installation

- **By Git**
```bash
git clone https://github.com/Zukunter/Nexus.git
cd Nexus
cargo install --path .
```

- **By cargo**
```bash
cargo install nexus
```
---

# Example

If in the Rust's file main.rs is
```rust
fn main() {
    //nexus -i ./variables.rs
    println!("{msg_from_other_file}");
}
```
and in ./variables.rs
```rust
    let msg_from_other_file: &str = "Hi from other file in nexus";
```
after executing nexus -i main.rs -o ../proyect/src/main.rs you will find in that output file :
```rust
fn main() {
    let msg_from_other_file: &str = "Hi from other file in nexus";
    println!("{msg_from_other_file}");
}
```
