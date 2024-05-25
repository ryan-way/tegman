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

resource "aws_lambda_function" "temperature_lambda" {
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

resource "aws_api_gateway_rest_api" "temperature_api" {
  name = "temperature_api"
}

resource "aws_api_gateway_resource" "temperature_api_resource" {
  path_part   = "resource"
  parent_id   = aws_api_gateway_rest_api.temperature_api.root_resource_id
  rest_api_id = aws_api_gateway_rest_api.temperature_api.id
}

resource "aws_api_gateway_method" "temperature_api_resource_post_method" {
  rest_api_id   = aws_api_gateway_rest_api.temperature_api.id
  resource_id   = aws_api_gateway_resource.temperature_api_resource.id
  http_method   = "POST"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "integration" {
  rest_api_id             = aws_api_gateway_rest_api.temperature_api.id
  resource_id             = aws_api_gateway_resource.temperature_api_resource.id
  http_method             = aws_api_gateway_method.temperature_api_resource_post_method.http_method
  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = aws_lambda_function.temperature_lambda.invoke_arn
}

# Lambda
resource "aws_lambda_permission" "apigw_lambda_permission" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.temperature_lambda.function_name
  principal     = "apigateway.amazonaws.com"

  # More: http://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-control-access-using-iam-policies-to-invoke-api.html
  source_arn = "arn:aws:execute-api:us-west-1:058264156666:${aws_api_gateway_rest_api.temperature_api.id}/*/${aws_api_gateway_method.temperature_api_resource_post_method.http_method}${aws_api_gateway_resource.temperature_api_resource.path}"
}

# IAM
data "aws_iam_policy_document" "apigw_lambda_assume_role" {
  statement {
    effect = "Allow"

    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }

    actions = ["sts:AssumeRole"]
  }
}

resource "aws_iam_role" "role" {
  name               = "myrole"
  assume_role_policy = data.aws_iam_policy_document.apigw_lambda_assume_role.json
}
