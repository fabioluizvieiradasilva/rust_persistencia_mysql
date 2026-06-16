#!/bin/bash

HOST="localhost"
PORT="3306"
USER="root"
SQL_FILE="restore.sql"

# Verifica se o arquivo existe
if [ ! -f "$SQL_FILE" ]; then
    echo "Erro: Arquivo $SQL_FILE não encontrado."
    exit 1
fi

echo "Executando script SQL..."

mysql \
    -h "$HOST" \
    -P "$PORT" \
    -u "$USER" \
    -p < "$SQL_FILE"

# Verifica se houve erro
if [ $? -eq 0 ]; then
    echo "Script executado com sucesso!"
else
    echo "Erro ao executar o script."
    exit 1
fi    