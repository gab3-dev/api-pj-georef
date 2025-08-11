# Attached options
alias dcu='docker compose up'
alias dcd='docker compose down'
alias dcr='docker compose down && docker compose up --force-recreate'

# Detached options
alias dcud='docker compose up -d'
alias dcrd='docker compose down && docker compose up -d --force-recreate'

dkrRm() {
  docker rm $(docker ps -qa)
}

dckRmi() {
  docker rmi $(docker images -qa)
}

alias fzf='fzf --tmux --preview="cat {}"'

fzf_view() {
  fzf --preview="cat {}"
}

nvim_fzf() {
  nvim $(fzf --preview="cat {}")
}
