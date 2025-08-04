# Attached options
alias dcu='docker compose up'
alias dcd='docker compose down'
alias dcr='docker compose down && docker compose up --force-recreate'

# Detached options
alias dcud='docker compose up -d'
alias dcrd='docker compose down && docker compose up -d --force-recreate'

function dkrRm() {
  docker rm $(docker ps -a -q)
}

function dkrRmi() {
  docker rmi $(docker images -q)
}
