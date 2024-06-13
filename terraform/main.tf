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

resource "aws_dynamodb_table" "tegmen-table" {
  name         = "temperature"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "Id"

  attribute {
    name = "Id"
    type = "S"
  }

  attribute {
    name = "Hostname"
    type = "S"
  }

  attribute {
    name = "Date"
    type = "S"
  }

  global_secondary_index {
    name            = "Hostname"
    hash_key        = "Hostname"
    range_key       = "Date"
    projection_type = "ALL"
  }
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
