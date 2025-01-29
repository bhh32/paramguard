 # **ParamGuard**

 ParamGuard is a standalone configuration management tool designed to simplify and secure the handling of environment variables, application configurations, and key-value pairs. It features a **command-line interface (CLI)**, an **interactive Text User Interface (TUI)**, and a **graphical user interface (GUI)** to meet the needs of developers, DevOps engineers, and IT administrators.

 With built-in features like **archiving**, **retention policies**, **encryption**, and **schema validation**, ParamGuard ensures your configurations are organized, protected, and easy to manage.

 ---

 ## **Features**
 - **Environment Variable Management**:
   - Add, edit, group, export, and import environment variables.
- **Configuration File Support**:
   - Manage JSON, YAML, INI, and XML files with schema validation.
- **Key-Value Pair Management**:
   - Store and organize custom key-value pairs for your applications.
- **Archiving System**:
   - Safely archive configurations with configurable retention periods to prevent accidental deletion.
- **Interactive TUI**:
   - Navigate and edit configurations visually using arrow keys in a terminal-based interface.
- **Security Built-In**:
   - Encrypt sensitive data locally using AES-GCM encryption.
   - Protect access with password-based authentication using Argon2 hashing.

---

## **Installation**

### Prerequisites
- Rust (latest stable version)
- SQLite (included by default)

### Build from Source
1. Clone the repository:

```bash
git clone https://github.com/OpenGuardianTech/paramguard.git
cd paramguard
```

2. Build the CLI/TUI binary:
```bash
 cargo build --release --package paramguard-cli
```

3. Build the GUI binary:
```bash
  cargo build --release --package paramguard-gui
```

4. Run the binaries:
- CLI/TUI: `./target/release/paramguard-cli`
- GUI: `./target/release/paramguard-gui`

---

## **Usage**

### CLI Examples
```bash
# Add an environment variable
paramguard-cli env add --name "API_KEY" --value "12345"

# List all environment variables in a set
paramguard-cli env list --set "development"

# Archive a configuration file
paramguard-cli archive --name "old-config"

# Restore an archived configuration file
paramguard-cli archive restore --name "old-config"

# Launch the interactive TUI
paramguard-cli tui
```

### GUI
1. Launch the GUI application (while in development):

```bash
./target/release/paramguard-gui
```
2. Use the intuitive graphical interface to manage configurations, edit files, or adjust settings.

---

## **Core Concepts**

### Archiving System
ParamGuard includes a robust archiving system to protect users from accidental deletions. When a configuration is archived:
    - It is moved to an "Archived" state but remains recoverable.
    - A retention period (default: 30 days) prevents deletion until it expires.
    - After the retention period, users can delete archived configurations manually or enable automatic cleanup.

### Security Features
ParamGuard ensures your configurations are safe with:

1. Local encryption using AES-GCM for sensitive data.
2. Password-based authentication using Argon2 hashing for secure access.

---

## **Configuration**

ParamGuard allows you to customize its behavior through settings:

### Example Settings Commands (CLI)

```bash
# Set retention period for archived files (in days)
paramguard-cli archive --set-retention-period 30
    
# Enable automatic deletion of archived files after retention period expires
paramguard-cli archive --set-auto-remove

# Disable automatic deletion of archived files after retention period
paramguard-cli archive --set-auto-remove=false

# View current settings
paramguard-cli settings show
```

---

## **Development Roadmap**

1. Core functionality as a shared library (`paramguard-core`).
2. CLI/TUI as a single binary (`paramguard-cli`).
3. GUI as a separate binary (`paramguard-gui`).
4. Advanced features such as schema validation for JSON/YAML/TOML/ENV/CFG/INI/NIX configurations and automatic cleanup of archived files.

