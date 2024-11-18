# rust-sound-flipper

### **MacOS**

```brew install lame```

#### build.rs
```
fn main() {
    println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
    println!("cargo:rustc-link-lib=mp3lame"); 
}
```

`cargo build`
`cargo run -- --input "path/to/file.[mp3, wav]"`