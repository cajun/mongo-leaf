task :log do
  `multitail -o beep_method:popup                               \
    -p o 10 -p l -cT ansi -l '$(DOCKER_COMPOSE) logs -f clippy'     \
    -p o 10 -p l -cT ansi -l '$(DOCKER_COMPOSE) logs -f lib'  \
    -p o 10 -p l -cT ansi -l '$(DOCKER_COMPOSE) logs -f nightly'`
end
