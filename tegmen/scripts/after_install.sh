OUTPUT=/var/log/code_deploy_scripts/$(date +%F-%H).log
date >> $OUTPUT
echo "install binary..." >> $OUTPUT
/root/.cargo/bin/cargo install --path /root/tegmen >> $OUTPUT || exit 1
echo "restarting cron..." >> $OUTPUT
crontab /root/tegmen/crontab >> $OUTPUT || exit 1
service cron restart >> $OUTPUT || exit 1