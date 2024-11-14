terraform {
  required_providers {
    aws = {
      source = "hashicorp/aws"
      version = "5.46.0"
    }
  }
}

# Configure the AWS Provider for workload deploy
provider "aws" {
  default_tags {
    tags = {
      Environment = var.environment
      Application = "tudigo"
      Owner       = "infra-terraform-team"
    }
  }
  region = "{{AWS_REGION}}"
}