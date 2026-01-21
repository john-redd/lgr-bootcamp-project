## Basic Usage

1. Configure credentials by running `aws configure export-credentials --format env-no-export --profile personal > .env`
2. Run terraform commands by injecting the contents of `.env` into the process.
e.g.
    - `dotenvx run -- terraform init`
    - `dotenvx run -- terraform validate`
    - `dotenvx run -- terraform plan`
    - `dotenvx run -- terraform apply`
    - `dotenvx run -- terraform output -json iam_secret_access_key`

## Cost Savings

To destroy ECS & ALB resources (saving money):
`dotenvx run -- terraform apply -var="enable_ecs=false"`
This will destroy the ECS cluster, services, tasks, load balancer, and related security groups, but leave your VPC, ECR repositories, and CICD IAM user intact.

To restore them later:
`dotenvx run -- terraform apply -var="enable_ecs=true"`
(Or simply terraform apply since the default is true)
