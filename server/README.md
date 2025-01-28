# UnityCatalog Server

A Rust implementation of the UnityCatalog server component, providing metadata management and catalog services.

## Overview

UnityCatalog Server is the backend component responsible for managing metadata, schemas, and catalog operations. This Rust implementation aims to provide a high-performance, memory-safe alternative to the original implementation.

## Features

- Metadata management for data assets
- Schema validation and enforcement
- Catalog operations (create, read, update, delete)
- REST API endpoints for client interactions
- Integration with storage backends

## Prerequisites

- Rust 1.75.0 or higher
- Cargo package manager

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/unity-catalog-rust
cd unity-catalog-rust/server
```

2. Build the project:
```bash
cargo build --release
```

## Configuration

Create a `config.toml` file in the `config` directory:

```toml
[server]
host = "127.0.0.1"
port = 8080

[storage]
backend = "local"  # or "s3", "azure", etc.
path = "/data/unity-catalog"
```

## Running the Server

1. Start the server in development mode:
```bash
cargo run
```

2. For production deployment:
```bash
./target/release/unity-catalog-server
```

## API Documentation

The server exposes the following REST endpoints:

- `GET /api/v1/catalogs` - List all catalogs
- `POST /api/v1/catalogs` - Create a new catalog
- `GET /api/v1/catalogs/{id}` - Get catalog details
- `PUT /api/v1/catalogs/{id}` - Update catalog
- `DELETE /api/v1/catalogs/{id}` - Delete catalog

For detailed API documentation, see `docs/api.md`.

## Testing

Run the test suite:

```bash
cargo test
```

For integration tests:

```bash
cargo test --features integration-tests
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Original UnityCatalog project contributors
- Rust community for excellent tools and libraries
- Contributors to this Rust implementation

## Contact

For questions and support:
- Create an issue in the GitHub repository
<!--- Join our Discord community [link]-->
- Email the maintainers at revanthshalonraj@gmail.com
