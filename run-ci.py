import subprocess

try: 
    subprocess.run(["docker", "--version"])
except: 
    print("Docker is not installed")
    exit(1)

subprocess.run(["docker", "build", ".", "-t", "rust_tcp_scanner_ci"])

subprocess.run(["docker", "run", "rust_tcp_scanner_ci"])
