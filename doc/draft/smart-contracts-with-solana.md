https://www.leewayhertz.com/build-solana-smart-contracts/

apt upgrade
apt update
apt install nodejs
apt install npm
apt install python3-pip
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sh -c "$(curl -sSfL https://release.solana.com/v1.5.7/install)"
source $HOME/.cargo/env
export PATH="/root/.local/share/solana/install/active_release/bin:$PATH"
export RUST_LOG=solana_runtime::system_instruction_processor=trace,solana_runtime::message_processor=debug,solana_bpf_loader=debug,solana_rbpf=debug
solana-test-validator --log

---

sudo yum upgrade

Ran out of disk space ...

   4  sudo yum upgrade
    5  sudo yum update
    6  sudo yum install nodejs
    7  sudo yum install npm
    8  sudo yum install python3-pip
    9  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   10  sh -c "$(curl -sSfL https://release.solana.com/v1.5.7/install)"
   11  sudo sh -c "$(curl -sSfL https://release.solana.com/v1.5.7/install)"
   12  df -k
   
---

I download and unpack the Maven tarball to $HOME/dist.

In my $HOME/.bash_profile I add:

`export MAVEN_HOME=$HOME/dist/apache-maven-3.8.5`



