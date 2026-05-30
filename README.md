# Nexus

Nexus is a lightweight tool designed to **nest files in absolutle any way you want**. 

---

## Atributes

- **Size** : 500 KB.
- **Language** : Completely written in Rust.
- **Multithread** : Yes.
- **Safe Memory Use** : The maxium amount of memory use for thread is 16KB.

---

## Wiki
- **[Commands](wiki/commands.md)** : General wiki for each avaible command.
- **[Nexus Default File](wiki/nexus.md)** : Define a nexus file so you don't have to write anymore than just nexus.

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

Let's say you have 3 files :

### start.rs
```rust
//nexus -i ./functions.rs
fn main() {
    //nexus -i ./variables.rs
    print_msg(msg_from_other_file);
}
```

### variables.rs
```rust
    let msg_from_other_file: &str = "Hi from other file in nexus";
```

### functions.rs
```rust
fn print_msg(msg: &str) {
    println!("Preparing... {msg}");
}
```
--- 

After executing the next command you will find the file in ../output_path that contains : 

```bash
nexus -i start.rs -o ../output_path -p //
```

```rust
fn print_msg(msg: &str) {
    println!("Preparing... {msg}");
}

fn main() {
    let msg_from_other_file: &str = "Hi from other file in nexus";
    print_msg(msg_from_other_file);
}
```

You could also execute this and see how cargo run the program

```bash
nexus -i start.rs -o ../output_path -p // -e cargo run _
``` 
