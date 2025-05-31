# PassVault

## A Secure CLI Password Manager written in Rust

PassVault is a high-performance command-line password manager built with Rust that provides secure local password storage. Leveraging Rust's memory safety and performance benefits along with military-grade AES-256-GCM encryption, this tool offers both security and convenience for developers and security-conscious users who prefer terminal-based workflows.

## How to use PassVault

There are 2 main ways to interact with the application

### Managing Passwords

You can store and retrieve passwords securely by:

1. **Initializing the vault** - Set up your master key and encrypted storage
2. **Adding passwords** - Store new credentials with descriptions and metadata
3. **Retrieving passwords** - Access your stored credentials with the master key
4. **Listing entries** - View all stored password names and metadata

The application provides secure encryption and validation:
- **AES-256-GCM encryption** protects all stored passwords
- **Master key validation** ensures only authorized access
- **Local storage** keeps your data under your control
- **Base64 encoding** provides safe file storage format

### Interactive Mode

You can also use PassVault in interactive mode:
- Launch without arguments for guided prompts
- Enter password names and master key interactively
- Quick access to frequently used credentials
- User-friendly interface for command-line newcomers

## How to install PassVault

### Prerequisites

Make sure you have Rust installed on your system. You can get it from [rustup.rs](https://rustup.rs/).

### From source

You can clone this repository using git:

```bash
git clone https://github.com/Adel-Ayoub/PassVault.git
cd PassVault
```

After cloning, you can use the following commands:

| Command | Description |
| ------- | ----------- |
| **`cargo build`** | Compiles the project in debug mode |
| **`cargo build --release`** | Builds an optimized release version |
| **`cargo run`** | Compiles and runs the application |
| **`cargo test`** | Runs all unit tests |
| **`cargo clean`** | Removes build artifacts |

### Binary Release

> W.I.P - Pre-compiled binaries will be available for download

## Usage Examples

```bash
# Initialize your password vault
passvault --init

# Add a new password
passvault -a "mypassword123" -n "gmail" -k "your-master-key" -d "My Gmail account"

# Retrieve a password
passvault -n "gmail" -k "your-master-key"

# List all passwords
passvault --list

# Interactive mode (no arguments)
passvault
```

## Known Issues

> There are currently no known issues. Please report any bugs you encounter.

## More Information about PassVault

### The Features

PassVault includes the following functionality:

#### Core Features

| Feature | Description | Scope |
| ------- | ----------- | ----- |
| **AES-256-GCM Encryption** | Military-grade encryption for password storage | All stored passwords |
| **Master Key Authentication** | Hash-based validation of the original master key | Security verification |
| **Local File Storage** | Organized directory structure for encrypted data | Filesystem-based vault |
| **Interactive CLI** | User-friendly command-line interface | All operations |
| **Metadata Support** | Store descriptions, dates, and organizational info | Enhanced password management |

#### Command Options

| Command | Description | Function |
| ------- | ----------- | -------- |
| **`-i, --init`** | Initialize new password vault | Creates master key and storage |
| **`-a, --append`** | Add new password | Stores encrypted password |
| **`-n, --name`** | Specify password name | Identifies stored credentials |
| **`-k, --key`** | Provide master key | Authentication for operations |
| **`-d, --description`** | Add password description | Metadata storage |
| **`-l, --list`** | List all passwords | View stored entries |
| **`-r, --reset`** | Reset password vault | Clear all data |

### Technical Details

The application is built using:

- **Rust** - Systems programming language for performance and safety
- **AES-GCM** - Authenticated encryption for maximum security
- **Base64** - Safe encoding for encrypted data storage
- **MD5 Hashing** - Master key validation and verification

### Security Features

PassVault implements multiple layers of security:

- **AES-256-GCM encryption** with unique nonces for each password
- **Master key validation** prevents unauthorized access attempts
- **Local storage only** - no cloud dependencies or data transmission
- **Secure key derivation** with random key generation
- **Memory safety** through Rust's ownership system
