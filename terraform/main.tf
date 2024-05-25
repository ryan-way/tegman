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
  engine_mode             = "serverless"
  availability_zones      = ["us-west-1a", "us-west-1b", "us-west-1a"]
  database_name           = "tegmen"
  master_username         = "root"
  master_password         = "rootroot"
  backup_retention_period = 5
  enable_http_endpoint    = true
  preferred_backup_window = "07:00-09:00"
  skip_final_snapshot     = true
  scaling_configuration {
    min_capacity = 2
  }
}

data "aws_iam_policy_document" "tegmen_lambda_assume_role_policy" {
  statement {
    effect = "Allow"

    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }

    actions = ["sts:AssumeRole"]
  }
}

resource "aws_iam_role" "tegmen_lambda_role" {
  name               = "tegmen_lambda_role"
  assume_role_policy = data.aws_iam_policy_document.tegmen_lambda_assume_role_policy.json
}

data "archive_file" "temperature_lambda_archive_file" {
  type        = "zip"
  source_file = "../target/lambda/temperature_lambda/bootstrap"
  output_path = "bootstrap.zip"
}

resource "aws_lambda_function" "tegman_lambda" {
  # If the file is not in the current working directory you will need to include a
  # path.module in the filename.

  filename         = "bootstrap.zip"
  function_name    = "temperature_lambda"
  role             = aws_iam_role.tegmen_lambda_role.arn
  source_code_hash = data.archive_file.temperature_lambda_archive_file.output_base64sha256
  handler          = "bootstrap"
  runtime          = "provided.al2023"

  environment {
    variables = {
      foo = "bar"
    }
  }
}
