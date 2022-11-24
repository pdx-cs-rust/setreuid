cargo build
sudo cp target/debug/setreuid .
sudo chown root ./setreuid
sudo chmod 4111 ./setreuid
./setreuid
