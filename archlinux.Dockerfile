from archlinux/base
run pacman -Suy --noconfirm git cargo gcc
run git clone https://github.com/davethiede/nom-u128.git
workdir nom-u128
run cargo build
