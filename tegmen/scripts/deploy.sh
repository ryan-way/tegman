readonly TARGET_HOST=192.168.4.36
readonly TARGET_PATH=/home/rway/tegmen
readonly TARGET_ARCH=arm-unknown-linux-gnueabihf
readonly SOURCE_PATH=../target/${TARGET_ARCH}/release/tegmen

cargo build --release --target=${TARGET_ARCH}
rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
ssh -t ${TARGET_HOST} sudo setcap 'cap_sys_nice=eip' ${TARGET_PATH}
ssh -t ${TARGET_HOST} ${TARGET_PATH}