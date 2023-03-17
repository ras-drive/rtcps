import subprocess

container_name = "rust_tcp_scanner_ci"

try:
    subprocess.run(["docker", "--version"])
except: 
    print("Docker is not installed")
    exit(1)

subprocess.run(["docker", "build", ".", "-t", container_name])

subprocess.run(["docker", "run", container_name])

exit(0)