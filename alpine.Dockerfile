# Interactive
#   docker run --rm -it alpine ash
# Container build
#   docker build -t nom .
from alpine
run apk add cargo git
run git clone https://github.com/davethiede/nom-u128.git
workdir nom-u128
run cargo build
