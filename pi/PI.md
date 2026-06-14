# Raspberry Pi Setup

Einmalige Vorbereitung eines frisch aufgesetzten Raspberry Pi 4B (Debian/Raspberry Pi OS).

## Voraussetzungen auf dem Mac

Der SSH-Key muss auf dem Pi hinterlegt sein:

```bash
ssh-copy-id markgrafen-pi
```

## 1. Rust installieren (auf dem Pi)

```bash
ssh markgrafen-pi
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env
```

## 2. Build-Abhängigkeiten installieren (auf dem Pi)

SQLite und weitere C-Bibliotheken werden beim Kompilieren benötigt:

```bash
sudo apt update && sudo apt install -y \
    libsqlite3-dev \
    libssl-dev \
    pkg-config \
    build-essential
```

## 3. nginx installieren und konfigurieren (auf dem Pi)

```bash
sudo apt install -y nginx
```

Die Nginx-Konfiguration wird automatisch vom `deploy_to_pi.sh`-Skript des Dashboard-Projekts kopiert und aktiviert. Es ist keine manuelle Konfiguration erforderlich.

## 4. Erstes Deployment (auf dem Mac)

Zurück auf dem Mac, im Projektverzeichnis:

```bash
pi/deploy_to_pi.sh --init
```

Dies kopiert den Quellcode und die Datenbank (`data/recipes.db`) auf den Pi.

## 5. Ersten Build und Service-Start

```bash
pi/update_pi.sh
```

Dieses Script kompiliert die Anwendung auf dem Pi, richtet den systemd Service ein und startet ihn. Ab jetzt startet die App automatisch beim Booten.

## Danach: Updates einspielen

Für jedes spätere Update genügt:

```bash
pi/update_pi.sh
```

Die Datenbank auf dem Pi wird dabei nie überschrieben.

## Wichtig

Die Nginx-Konfiguration wird automatisch vom `deploy_to_pi.sh`-Skript des Dashboard-Projekts kopiert und aktiviert. Es ist keine manuelle Konfiguration erforderlich. Die Konfiguration wird in `/home/${PI_USER}/nginx-config/` gespeichert und von dort aus aktiviert.
