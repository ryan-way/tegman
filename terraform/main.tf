terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "5.50.0"
    }
  }
}

provider "aws" {
  region  = "us-west-1"
  profile = "tegmen"
}

resource "aws_rds_cluster" "tegmen_db" {
  cluster_identifier      = "tegmen"
  engine                  = "aurora-postgresql"
  engine_mode             = "provisioned"
  availability_zones      = ["us-west-1a", "us-west-1b", "us-west-1a"]
  database_name           = "tegmen"
  master_username         = "root"
  master_password         = "rootroot"
  backup_retention_period = 5
  enable_http_endpoint    = true
  preferred_backup_window = "07:00-09:00"
  skip_final_snapshot     = true
}

resource "aws_rds_cluster_instance" "cluster_instances" {
  count               = 1
  identifier          = "tegmen-cluster-instance-${count.index}"
  cluster_identifier  = aws_rds_cluster.tegmen_db.id
  instance_class      = "db.r5.large"
  engine              = aws_rds_cluster.tegmen_db.engine
  engine_version      = aws_rds_cluster.tegmen_db.engine_version
  publicly_accessible = true
}

module "temperature_lambda" {
  source = "./lambda"
  name   = "temperature_lambda"
}

module "temperature_api" {
  source            = "./api-gateway"
  name              = "temperature_api"
  function_name     = module.temperature_lambda.function_name
  lambda_arn        = module.temperature_lambda.arn
  lambda_invoke_arn = module.temperature_lambda.invoke_arn
}

module "system_controller" {
  source = "./lambda"
  name   = "system_controller"
}

resource "aws_cloudwatch_event_rule" "every_5_minutes" {
  name                = "every_5_minutes"
  description         = "Runs every 5 minutes"
  schedule_expression = "rate(5 minutes)"
}

resource "aws_lambda_permission" "allow_lambda" {
  statement_id  = "AllowExecutionFromCloudWatch"
  action        = "lambda:InvokeFunction"
  function_name = module.system_controller.function_name
  principal     = "events.amazonaws.com"
  source_arn    = aws_cloudwatch_event_rule.every_5_minutes.arn
}

resource "aws_cloudwatch_event_target" "event_bridge_target" {
  rule = aws_cloudwatch_event_rule.every_5_minutes.name
  arn  = module.system_controller.arn
}

module "codedeploy" {
  source = "./codedeploy"
}
