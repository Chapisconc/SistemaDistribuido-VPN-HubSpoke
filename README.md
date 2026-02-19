# Sistema Distribuido: Cálculo de Mandelbrot sobre VPN

Este repositorio contiene la infraestructura y el prototipo de software para un sistema distribuido desarrollado para la materia de *Uso, adaptación y explotación de sistemas operativos* de la Universidad de Guadalajara (CUCEI).

## 👥 Equipo
* **Jorge Iván Ramírez Llamas**
* **Sofía Gómez Altón**
* **Cristopher Said Ramírez Ruiz** (Coordinador)
* **Luis Rogelio Ríos Arellano**

## 📂 Estructura del Repositorio
El proyecto sigue la siguiente estructura de directorios:

* `/vpn`: Archivos de configuración de WireGuard (plantillas sanitizadas sin llaves privadas).
* `/docker`: Orquestación de contenedores (Dockerfile y docker-compose).
* `/rust`: Código fuente del sistema distribuido (Coordinador y Workers).
* `/docs`: Documentación técnica y reportes de avance.

## 🛠 Requisitos Previos
* **Sistema Operativo:** Linux (Ubuntu 20.04/22.04 o WSL2).
* **Contenedores:** Docker y Docker Compose v2+.
* **Lenguaje:** Rust (Cargo) instalado.
* **Red:** WireGuard (`wireguard-tools`).

## 🚀 Instrucciones de Despliegue

### 1. Configuración de la VPN
Cada nodo debe generar sus propias llaves privadas. Las configuraciones de ejemplo se encuentran en la carpeta `/vpn`.

```bash
# Instalar WireGuard
sudo apt install wireguard

# Generar llaves (en cada nodo)
wg genkey | tee privatekey | wg pubkey > publickey

# Levantar la interfaz (requiere configurar wg0.conf con las llaves reales)
sudo wg-quick up wg0
2. Levantar Contenedores
El entorno de ejecución se gestiona con Docker Compose. Asegúrese de estar en la carpeta /docker.
code
Bash
cd docker
docker compose up -d
docker ps  # Verificar que los contenedores (hub/worker) estén activos
3. Compilar y Ejecutar Sistema Distribuido (Rust)
El proyecto utiliza un workspace de Rust. Para probar el prototipo "Hello Distributed":
code
Bash
cd rust
cargo build --release
cargo run
