terraform {
  cloud {
    organization = "tudigo"

    workspaces {
      tags = ["{{organization}}-aws-<component>"]
    }
  }
}