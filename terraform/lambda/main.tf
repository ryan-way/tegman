variable "name" { type = string }

data "aws_iam_policy_document" "logging_policy_document" {
  statement {
    effect = "Allow"

    actions = [
      "logs:CreateLogGroup",
      "logs:CreateLogStream",
      "logs:PutLogEvents",
    ]

    resources = ["arn:aws:logs:*:*:*"]
  }
}

resource "aws_iam_policy" "lambda_logging_policy" {
  name        = "${var.name}_lambda_logging"
  path        = "/"
  description = "IAM policy for logging from a lambda"
  policy      = data.aws_iam_policy_document.logging_policy_document.json
}


data "aws_iam_policy_document" "assume_role_policy" {
  statement {
    effect = "Allow"

    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }

    actions = ["sts:AssumeRole"]
  }
}

resource "aws_iam_role" "lambda_role" {
  name               = "${var.name}_role"
  assume_role_policy = data.aws_iam_policy_document.assume_role_policy.json
}


resource "aws_iam_role_policy_attachment" "loggin_policy_attachment" {
  role       = aws_iam_role.lambda_role.name
  policy_arn = aws_iam_policy.lambda_logging_policy.arn
}

data "archive_file" "lambda_archive_file" {
  type        = "zip"
  source_file = "../target/lambda/${var.name}/bootstrap"
  output_path = "${var.name}_bootstrap.zip"
}

resource "aws_lambda_function" "lambda" {
  # If the file is not in the current working directory you will need to include a
  # path.module in the filename.

  filename         = "${var.name}_bootstrap.zip"
  function_name    = var.name
  role             = aws_iam_role.lambda_role.arn
  source_code_hash = data.archive_file.lambda_archive_file.output_base64sha256
  handler          = "bootstrap"
  runtime          = "provided.al2023"

  environment {
    variables = {
      foo = "bar"
    }
  }
}

output "function_name" { value = aws_lambda_function.lambda.function_name }
output "invoke_arn" { value = aws_lambda_function.lambda.invoke_arn }
output "arn" { value = aws_lambda_function.lambda.arn }