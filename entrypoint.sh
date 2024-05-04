#!/bin/bash
set -e  # Hacer que el script falle en caso de cualquier error

# Iniciar el servidor SSH
service ssh start

# Esperar a que la base de datos esté disponible
dockerize -wait tcp://${DATABASE_HOST}:3306 -timeout 180s

# Configurar y ejecutar las migraciones con Diesel
diesel setup

# Iniciar la aplicación
exec "$@"
