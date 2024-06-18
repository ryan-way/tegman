OUTPUT=/var/log/code_deploy_scripts/$(date +%F-%H).log
{
  date
  echo "### BEFORE INSTALL ###"
  echo "## Fetching release..." 
  wget https://github.com/ryan-way/tegman/releases/latest/download/tegmen.zip || exit 1
  echo "## Unpacking..."
  unzip -o tegmen.zip
} >>$OUTPUT 2>&1