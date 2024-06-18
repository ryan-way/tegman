OUTPUT=/var/log/code_deploy_scripts/$(date +%F-%H).log
{
  date
  echo "### AFTER INSTALL ###"
  echo "## Restarting cron..."
  crontab tegmen/crontab || exit 1
  service cron restart || exit 1
} >> $OUTPUT