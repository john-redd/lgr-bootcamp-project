terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.92"
    }
  }

  required_version = ">= 1.2"

  backend "s3" {
    bucket = "lgr-bootcamp-project-tf" # Replace with your S3 bucket name
    key    = "ecr-registry/terraform.tfstate"
    region = "us-east-1"
    use_lockfile = true
  }
}
