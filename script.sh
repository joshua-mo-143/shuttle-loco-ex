db_connection=$(cargo shuttle resource list --show-secrets | awk -F '┆' '!/Connection string/ && NR>2 {gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2); if ($2) print $2}' | sed 's/ │//')

sed -i -E "s|postgres:\/\/[A-Za-z0-9@:_].*|$db_connection|" ./config/development.yaml
