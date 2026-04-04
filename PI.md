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
    pkg-config \
    build-essential
```

## 3. nginx installieren und konfigurieren (auf dem Pi)

```bash
sudo apt install -y nginx
```

Konfiguration anlegen:

```bash
sudo nano /etc/nginx/sites-available/rezepte
```

Inhalt:

```nginx
server {
    listen 80;
    server_name markgrafen-pi markgrafen-pi.local _;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

Aktivieren:

```bash
sudo ln -s /etc/nginx/sites-available/rezepte /etc/nginx/sites-enabled/
sudo rm /etc/nginx/sites-enabled/default
sudo nginx -t && sudo systemctl enable nginx && sudo systemctl start nginx
```

## 4. Erstes Deployment (auf dem Mac)

Zurück auf dem Mac, im Projektverzeichnis:

```bash
./deploy_to_pi.sh --init
```

Dies kopiert den Quellcode und die Datenbank (`data/recipes.db`) auf den Pi.

## 5. Ersten Build und Service-Start

```bash
./update_pi.sh
```

Dieses Script kompiliert die Anwendung auf dem Pi, richtet den systemd Service ein und startet ihn. Ab jetzt startet die App automatisch beim Booten.

## Danach: Updates einspielen

Für jedes spätere Update genügt:

```bash
./update_pi.sh
```

Die Datenbank auf dem Pi wird dabei nie überschrieben.
