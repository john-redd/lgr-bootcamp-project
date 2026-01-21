1. Configure credentials by running `aws configure export-credentials --format env-no-export --profile personal > .env`
2. Run terraform commands by injecting the contents of `.env` into the process.
e.g.
    - `dotenvx run -- terraform init`
    - `dotenvx run -- terraform validate`
    - `dotenvx run -- terraform plan`
    - `dotenvx run -- terraform apply`
    - `dotenvx run -- terraform output -json iam_secret_access_key`
