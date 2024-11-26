Steps to create a clone Unix Command (ccwc)

- Compile the program
```bash 
cargo build --release
```

- Move the executable to a directory in your PATH
```bash
sudo cp ~/projects/wc_cli/target/release/ccwc /usr/local/bin/
```

- Verify the installation
```bash
ccwc -c test.txt
```

- Optional: Create a symbolic link
```bash
sudo ln -s ~/projects/wc_cli/target/release/ccwc /usr/local/bin/ccwc
```

- Check if it is working
```bash
which ccwc
```


