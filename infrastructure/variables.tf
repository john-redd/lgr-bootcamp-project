variable "aws_region" {
  description = "AWS region"
  type        = string
  default     = "us-east-1"
}

variable "ecr_auth_service_repo_name" {
  description = "Name of the ECR repository for the Auth Service"
  type        = string
  default     = "auth-service"
}

variable "ecr_app_service_repo_name" {
  description = "Name of the ECR repository for the App Service"
  type        = string
  default     = "app-service"
}

variable "iam_user_name" {
  description = "Name of the IAM user for ECR access"
  type        = string
  default     = "ecr-cicd-user"
}

variable "enable_ecs" {
  description = "Enable ECS and ALB resources"
  type        = bool
  default     = true
}
