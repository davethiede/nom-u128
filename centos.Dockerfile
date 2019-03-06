from centos
run yum install -y epel-release
run yum install -y git cargo
workdir /root/
run git clone https://github.com/davethiede/nom-u128.git
run cd nom-u128; cargo build
