OUTPUT=/var/log/code_deploy_scripts/$(date +%F-%H).log
{
  date
  echo "### BEFORE INSTALL ###"
  echo "## Fetching release..." 
  wget https://github.com/ryan-way/tegman/releases/latest/download/tegmen.zip -O /root/tegmen.zip || exit 1
  echo "## Unpacking..."
  unzip -o /root/tegmen.zip || exit 1
  echo "### AFTER INSTALL ###"
  echo "## Restarting cron..."
  crontab tegmen/crontab || exit 1
  service cron restart || exit 1
} >> $OUTPUT 2>&1