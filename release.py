import subprocess

# check if cargo is installed
try: 
    subprocess.run(["cargo", "--version"])
except: 
    print("No cargo executable found")
    exit(1)

# check if cargo deb is installed
try: 
    subprocess.run(["cargo", "deb", "--version"])
except: 
    print("Cargo deb not found, it can be found at https://github.com/kornelski/cargo-deb")
    exit(1)



# run debian build
subprocess.run(["cargo", "deb", "-o", "target"])


exit(0)