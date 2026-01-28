# meeting-meter

A CLI tool to calculate the cost of meetings in real-time or retroactively.

## Features

- **Live Mode**: Start a real-time meeting timer that shows running costs
- **Retroactive Mode**: Log past meetings with start/end times or duration
- **Role-based Pricing**: Configure hourly rates for different roles (engineers, PMs, designers, etc.)
- **Meeting History**: Track all your meetings and view past costs
- **Reports**: Generate weekly, monthly, or all-time spending reports
- **Fun Equivalents**: See meeting costs in terms of Netflix subscriptions, burritos, etc.

## Releases

Pre-compiled binaries for Linux, macOS, and Windows are available on the [GitHub Releases](https://github.com/josh-spratt/meeting-meter/releases) page.

## Installation

### For End-Users (Recommended)

1.  Go to the [GitHub Releases](https://github.com/josh-spratt/meeting-meter/releases) page.
2.  Download the latest binary for your operating system.
3.  Place it in a directory that is in your system's `PATH`.

### From Source (For Developers)

```bash
cargo install --path .
```

## Quick Start

### Configure Rates (Optional)

Default rates are provided, but you can customize them:

```bash
# Set custom rate for a role
meeting-meter config set-rate engineer 120

# View all configured rates
meeting-meter config list-rates

# Reset to defaults
meeting-meter config reset
```

### Log a Past Meeting

```bash
# Using start and end times
meeting-meter log --start "2pm" --end "3:30pm" --engineers 3 --pms 1

# Using duration
meeting-meter log --duration "90m" --engineers 5 --directors 1

# Mix and match roles
meeting-meter log --duration "1h" --senior_engineers 2 --designers 1 --vps 1
```

### Start a Live Meeting

```bash
meeting-meter start --engineers 4 --pms 1 --designers 2
```

### View History and Reports

```bash
# View recent meetings
meeting-meter history --limit 10

# Generate reports
meeting-meter report --week
meeting-meter report --month
meeting-meter report --all
```

## Default Hourly Rates

| Role | Rate (USD/hr) |
|------|---------------|
| Engineer | $100 |
| Senior Engineer | $130 |
| Staff Engineer | $160 |
| Principal Engineer | $200 |
| Product Manager | $120 |
| Senior PM | $150 |
| Director of PM | $200 |
| Designer | $110 |
| Senior Designer | $140 |
| Analyst | $90 |
| Senior Analyst | $115 |
| Director | $200 |
| VP | $300 |
| Executive | $400 |
| Generic | $100 |

## Development Roadmap

- [x] **Phase 1**: Core foundation (config, CLI, models, storage)
- [x] **Phase 2**: Retroactive mode implementation
- [ ] **Phase 3**: Live mode with TUI
- [ ] **Phase 4**: Reporting and history
- [ ] **Phase 5**: Polish (equivalents, multipliers, formatting)

## Contributing

Contributions welcome! This is a fun project to learn Rust and build something useful.

## License

Apache 2.0
