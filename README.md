# Nexus

Nexus is a lightweight tool designed to **nest files in absolutle any way you want**. 

---

## Caracteristics

- **Size** : 500 KB.
- **Language** : Completely written in Rust.
- **Multithread** : Yes.

---

## Wiki
- **[Nexus Center File](wiki/nexus.md)** : Define a nexus file so you don't have to write anymore than just nexus.
- **[Simple Commands](wiki/commands.md)** : General wiki for each avaible command.

--- 

##  Installation

- By Git
```bash
git clone https://github.com/Zukunter/Nexus.git
cd Nexus
cargo install --path .
```

- By cargo
```bash
cargo install nexus
```
---

# Example

If in the Rust's file main.rs is :
```rust
fn main() {
    //nexus -i ./variables.rs
    println!("{msg_from_other_file}");
}
```
and in ./variables.rs is :
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
