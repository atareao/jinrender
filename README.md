# jinrender

[![Crates.io](https://img.shields.io/crates/v/jinrender.svg)](https://crates.io/crates/jinrender)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/github/actions/workflow/status/atareao/jinrender/ci.yml?branch=main)](https://github.com/atareao/jinrender/actions)

Una herramienta de línea de comandos simple y potente para renderizar plantillas Jinja2 utilizando variables de entorno del sistema.

## 📋 Tabla de Contenidos

- [Características](#-características)
- [Instalación](#-instalación)
- [Uso](#-uso)
- [Ejemplos](#-ejemplos)
- [Opciones de Línea de Comandos](#%EF%B8%8F-opciones-de-línea-de-comandos)
- [Casos de Uso](#-casos-de-uso)
- [Contribuir](#-contribuir)
- [Licencia](#-licencia)
- [Changelog](#-changelog)

## ✨ Características

- 🚀 **Rápido y ligero**: Escrito en Rust para máximo rendimiento
- 🔧 **Simple de usar**: Interfaz de línea de comandos intuitiva
- 🌍 **Variables de entorno**: Acceso automático a todas las variables del sistema
- 📄 **Plantillas Jinja2**: Soporte completo para sintaxis de plantillas Jinja2
- 🎯 **Sin dependencias**: Binario autónomo sin necesidad de runtimes adicionales
- 💻 **Multiplataforma**: Compatible con Linux, Windows y macOS
- 🧪 **Bien testado**: Suite completa de tests unitarios e integración
- 📊 **Benchmarks incluidos**: Medición de rendimiento y optimización continua

## 🚀 Instalación

### Desde Releases (Recomendado)

1. Descarga la última versión desde las [releases](https://github.com/atareao/jinrender/releases)
2. Extrae el archivo descargado
3. Mueve el binario a una ubicación en tu `PATH`

#### Linux/macOS

```bash
# Descargar y extraer
wget https://github.com/atareao/jinrender/releases/latest/download/jinrender-linux.tar.gz
tar -xzf jinrender-linux.tar.gz

# Mover al PATH
sudo mv jinrender /usr/local/bin/

# Verificar instalación
jinrender --version
```

#### Windows

1. Descarga `jinrender-windows.zip`
2. Extrae el archivo en una carpeta de tu elección
3. Añade la carpeta al PATH del sistema

### Desde Crates.io

```bash
cargo install jinrender
```

### Compilar desde el código fuente

```bash
# Clonar el repositorio
git clone https://github.com/atareao/jinrender.git
cd jinrender

# Compilar
cargo build --release

# El binario estará en target/release/jinrender
```

## 📖 Uso

La sintaxis básica de `jinrender` es simple:

```bash
jinrender -j <template_file> -o <output_file>
```

### Ejemplo Básico

1. **Crear una plantilla Jinja2**:

```jinja
<!-- template.jinja -->
Hola, {{ env.USERNAME }}!
Tu directorio home es: {{ env.HOME }}
Fecha actual: {{ env.DATE | default("No disponible") }}
```

2. **Establecer variables de entorno** (opcional):

```bash
export DATE=$(date)
```

3. **Ejecutar jinrender**:

```bash
jinrender -j template.jinja -o output.txt
```

4. **Resultado**:

```txt
Hola, atareao!
Tu directorio home es: /home/atareao
Fecha actual: Fri Feb 14 10:30:00 UTC 2026
```

## 💡 Ejemplos

### Generación de Archivos de Configuración

**Plantilla `config.jinja`**:

```jinja
[database]
host = {{ env.DB_HOST | default("localhost") }}
port = {{ env.DB_PORT | default("5432") }}
username = {{ env.DB_USER }}
password = {{ env.DB_PASS }}

[server]
bind = {{ env.SERVER_BIND | default("0.0.0.0") }}
port = {{ env.SERVER_PORT | default("8080") }}
```

**Comando**:

```bash
export DB_HOST="production.db.com"
export DB_USER="myapp"
export DB_PASS="secret123"
jinrender -j config.jinja -o app.conf
```

### Generación de Documentos SVG

**Plantilla `cover.jinja`**:

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="800" height="600">
  <rect width="800" height="600" fill="#2c3e50"/>
  <text x="400" y="200" font-family="Arial" font-size="48" fill="white" text-anchor="middle">
    {{ env.PODCAST_TITLE }}
  </text>
  <text x="400" y="300" font-family="Arial" font-size="32" fill="#ecf0f1" text-anchor="middle">
    Episodio {{ env.EPISODE_NUMBER }}
  </text>
  <text x="400" y="400" font-family="Arial" font-size="24" fill="#bdc3c7" text-anchor="middle">
    {{ env.PUBLISH_DATE }}
  </text>
</svg>
```

### Automatización con Scripts

```bash
#!/bin/bash
# generate_covers.sh

export PODCAST_TITLE="Mi Podcast Tech"
export PUBLISH_DATE=$(date +"%d de %B de %Y")

for episode in {1..10}; do
    export EPISODE_NUMBER=$episode
    jinrender -j cover_template.jinja -o "covers/episode_${episode}.svg"
done
```

## ⚙️ Opciones de Línea de Comandos

| Opción                | Descripción                                  | Requerido |
| --------------------- | -------------------------------------------- | --------- |
| `-j, --jinja <FILE>`  | Archivo de plantilla Jinja2 de entrada       | ✅        |
| `-o, --output <FILE>` | Archivo de salida donde guardar el resultado | ✅        |
| `-h, --help`          | Muestra ayuda                                | ❌        |
| `-V, --version`       | Muestra la versión                           | ❌        |

### Ayuda del Comando

```bash
jinrender --help
```

```
Render jinja templates

Usage: jinrender [OPTIONS] --jinja <FILE> --output <FILE>

Options:
  -j, --jinja <FILE>   Set the jinja file
  -o, --output <FILE>  Set the output file
  -h, --help           Print help
  -V, --version        Print version
```

## 🎯 Casos de Uso

### 1. **Generación de Portadas de Video/Podcast**

Ideal para crear portadas dinámicas usando plantillas SVG con información de episodios almacenada en variables de entorno.

### 2. **Archivos de Configuración**

Genera archivos de configuración para diferentes entornos (desarrollo, producción) usando variables de entorno específicas.

### 3. **Documentación Dinámica**

Crea documentación que se actualiza automáticamente con información del sistema o proyecto.

### 4. **Plantillas de Email**

Genera contenido de correos electrónicos personalizados usando datos del entorno.

### 5. **Archivos de Deploy**

Crea archivos de configuración de despliegue (Docker, Kubernetes) adaptados al entorno.

## � Testing y Desarrollo

### Ejecutar Tests

```bash
# Tests unitarios
cargo test

# Tests de integración
cargo test --test integration_tests

# Benchmarks de rendimiento
cargo test --bench benchmarks

# Todos los tests con salida detallada
cargo test -- --nocapture
```

### Coverage de Tests

El proyecto incluye:

- **Tests unitarios**: Para funciones individuales
- **Tests de integración**: Para el comportamiento completo de la aplicación
- **Tests de benchmarks**: Para medir y garantizar el rendimiento
- **Tests de casos edge**: Para manejo de errores y situaciones límite

### Métricas de Rendimiento

- ⚡ Renderizado simple: ~1000 renders/segundo
- 🔄 Templates complejos: ~100 renders/segundo
- 📚 1000+ variables de entorno: <50ms por render
- 💾 Recolección de env vars: <5ms

## �🤝 Contribuir

¡Las contribuciones son bienvenidas! Si quieres contribuir al proyecto:

1. **Fork** el repositorio
2. **Crea** una rama para tu feature (`git checkout -b feature/nueva-caracteristica`)
3. **Commit** tus cambios (`git commit -am 'Añadir nueva característica'`)
4. **Push** a la rama (`git push origin feature/nueva-caracteristica`)
5. **Crea** un Pull Request

### Desarrollo Local

```bash
# Clonar el repositorio
git clone https://github.com/atareao/jinrender.git
cd jinrender

# Ejecutar tests
cargo test

# Ejecutar benchmarks
cargo test --bench benchmarks

# Compilar en modo desarrollo
cargo build

# Compilar en modo release
cargo build --release

# Ejecutar con cargo
cargo run -- -j ejemplo.jinja -o salida.txt

# Verificar código
cargo clippy
cargo fmt
```

### Reportar Issues

Si encuentras un bug o tienes una sugerencia:

- Crea un [issue](https://github.com/atareao/jinrender/issues/new)
- Incluye información detallada sobre el problema
- Si es posible, incluye un ejemplo reproducible

## 📄 Licencia

Este proyecto está licenciado bajo la Licencia MIT - consulta el archivo [LICENSE](LICENSE) para más detalles.

```
MIT License

Copyright (c) 2024 Lorenzo Carbonell
```

## 📝 Changelog

### v0.1.2

- ✅ **Tests comprehensivos**: Suite completa de tests unitarios e integración
- 📊 **Benchmarks**: Medición de rendimiento y optimización
- 🔧 **Refactoring**: Código modularizado en lib.rs para mejor testabilidad
- 📦 **Versioning automático**: Versión obtenida automáticamente del Cargo.toml
- 📚 **Documentación mejorada**: README actualizado con información detallada

### v0.1.1

- 🎯 **Versión inicial**: Funcionalidad básica implementada
- 📄 **Plantillas Jinja2**: Soporte completo para renderizado
- 🌍 **Variables de entorno**: Integración con variables del sistema
- 💻 **CLI**: Interfaz de línea de comandos intuitiva

---

**Autor**: [Lorenzo Carbonell](https://github.com/atareao)  
**Repositorio**: [github.com/atareao/jinrender](https://github.com/atareao/jinrender)
