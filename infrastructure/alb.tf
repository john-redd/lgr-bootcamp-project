resource "aws_lb" "main" {
  count              = var.enable_ecs ? 1 : 0
  name               = "lgr-bootcamp-alb"
  internal           = false
  load_balancer_type = "application"
  security_groups    = [aws_security_group.alb_sg[0].id]
  subnets            = module.vpc.public_subnets

  enable_deletion_protection = false

  tags = {
    Name = "lgr-bootcamp-alb"
  }
}

resource "aws_lb_target_group" "app_service_tg" {
  count       = var.enable_ecs ? 1 : 0
  name        = "app-service-tg"
  port        = 8000
  protocol    = "HTTP"
  vpc_id      = module.vpc.vpc_id
  target_type = "ip"

  health_check {
    path                = "/"
    healthy_threshold   = 2
    unhealthy_threshold = 10
    timeout             = 60
    interval            = 300
    matcher             = "200"
  }
}

resource "aws_lb_target_group" "auth_service_tg" {
  count       = var.enable_ecs ? 1 : 0
  name        = "auth-service-tg"
  port        = 3000
  protocol    = "HTTP"
  vpc_id      = module.vpc.vpc_id
  target_type = "ip"

  health_check {
    path                = "/api/v1/health"
    healthy_threshold   = 2
    unhealthy_threshold = 10
    timeout             = 60
    interval            = 300
    matcher             = "200"
  }
}

resource "aws_lb_listener" "app_listener" {
  count             = var.enable_ecs ? 1 : 0
  load_balancer_arn = aws_lb.main[0].arn
  port              = "80"
  protocol          = "HTTP"

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.app_service_tg[0].arn
  }
}

resource "aws_lb_listener" "auth_listener" {
  count             = var.enable_ecs ? 1 : 0
  load_balancer_arn = aws_lb.main[0].arn
  port              = "3000"
  protocol          = "HTTP"

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.auth_service_tg[0].arn
  }
}
