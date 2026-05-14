# Contributing to MeteorPay 🌠

First off, thank you for considering contributing to MeteorPay! It's people like you that make MeteorPay such a great tool for the future of payments.

## 🚀 Getting Started

1. **Fork the Repository**: Create your own copy of the project.
2. **Clone the Repo**: `git clone https://github.com/CelestialCircles/MeteorPay-.git`
3. **Setup Environments**:
   - **Frontend**: Navigate to `/frontend`, run `npm install`.
   - **Contract**: Navigate to `/contract`, ensure you have Rust and Soroban CLI installed.
4. **Choose an Issue**: Look at our [ISSUES.md](./ISSUES.md) or the GitHub Issues tab.

## 🛠️ Development Workflow

1. **Create a Branch**: Use a descriptive name like `feature/qr-generator` or `fix/wallet-connection`.
2. **Write Code**: Follow our design aesthetics (premium, modern UI) and contract best practices.
3. **Test Your Changes**:
   - For contracts: `cargo test`
   - For frontend: `npm run lint` and manual UI verification.
4. **Submit a Pull Request**: Provide a clear description of your changes and link the relevant issue.

## 🎨 Design Guidelines (Frontend)

- **Aesthetics**: We aim for a "Premium" look. Use smooth gradients, consistent spacing, and modern typography (Inter/Outfit).
- **Dark Mode**: All components should support dark mode by default.
- **Responsiveness**: Mobile-first design is a must.

## 🦀 Contract Guidelines (Soroban)

- **Security**: Always validate inputs and check permissions for sensitive operations.
- **Gas Efficiency**: Optimize storage usage and avoid unnecessary computations.
- **Documentation**: Comment complex logic and use descriptive function names.

## 💬 Communication

If you have questions, feel free to open a Discussion or reach out to the maintainers.

Happy coding! 🚀
