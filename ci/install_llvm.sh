wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | sudo apt-key add -
sudo add-apt-repository -y 'deb http://apt.llvm.org/bionic/ llvm-toolchain-bionic-10 main'
sudo apt-get -q update
sudo apt-get -y install llvm-10
sudo apt-get -y install clang-10
find /usr -name libclang-10.so.1
export LLVM_CONFIG_PATH=/usr/bin/llvm-config-10
export LIBCLANG_PATH=/usr/lib/x86_64-linux-gnu/libclang-10.so.1
