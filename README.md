# Lamdon: Simplifying Lambda Function Initialization with Terraform

Lamdon is a command-line interface (CLI) tool designed to streamline the initialization process for Lambda functions, supporting runtimes such as Node.js and Go. With Lamdon, developers can swiftly set up the necessary folder structure for their chosen runtime, complete with pre-baked Infrastructure as Code (IaC) in Terraform. This eliminates the tedious manual setup process, empowering developers to focus on writing code rather than configuring environments.

# Grand Vision

Single tool to deploy SAAS functions in a cloud agnostic way with minimal configuration. 

# Features:

 - Effortless Initialization: Lamdon guides users through a simple prompt to select their desired runtime environment, automating the creation of the necessary project structure.
- Pre-configured IaC: Each runtime initialization includes pre-baked Terraform IaC, enabling seamless deployment and management of Lambda functions in the cloud.
- Streamlined Workflow: Leveraging Makefiles, Lamdon provides convenient commands for building, testing, and deploying Lambda functions, optimizing the development workflow.

Stay tuned as we continue to enhance Lamdon with additional features, such as a power tuner for optimizing Lambda function performance. Get started with Lamdon today and experience the ease of Lambda function initialization!

# Tech Stack:

- Clap simplifies the parsing and validation of command-line arguments and options, ensuring a smooth and error-free user experience.

- Ratatui enhances the CLI interface by enabling the creation of interactive components such as prompts and progress indicators, improving user engagement and usability.