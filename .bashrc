alias d-c-r='docker compose down && docker compose up -d --force-recreate'

alias dcu="docker compose up"
alias dcud="docker compose up -d"
alias dcd="docker compose down"
alias dcr="docker compose up --force-recreate"
alias dcrd="docker compose up --force-recreate -d"

dckRm() {
    docker rm $(docker ps -qa)
}
dckRmi() {
    docker rmi $(docker images -aq)
}